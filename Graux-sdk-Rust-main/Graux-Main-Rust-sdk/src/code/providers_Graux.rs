use std::convert::TryInto;

use ethers::prelude::*;
use ethers::providers::{
    Middleware, Provider, ProviderError, JsonRpcClient, Http, MiddlewareAction, MiddlewareContext,
};
use ethers::types::{Address, BlockId, U256};

use serde::{Deserialize, Serialize};

use graux::const::*;
use graux::logger::*;
use graux::util::*;

*Define custom error types for the provider*
#[derive(Debug, thiserror::Error)]
enum GrauxProviderError {
    #[error("No network detected")]
    NoNetworkDetected,

    #[error("Invalid network: {0}")]
    InvalidNetwork(String),

    #[error("Invalid API key: {0}")]
    InvalidApiKey(String),

    #[error(transparent)]
    ProviderError(#[from] ProviderError),
}

Custom implementation of GrauxProvider
pub struct GrauxProvider<C: JsonRpcClient + Clone> {
    provider: Provider<C>,
    api_key: String,
    max_retries: u32,
    batch_requests: bool,
}

impl<C: JsonRpcClient + Clone> GrauxProvider<C> {
    Initialize a new GrauxProvider instance
    pub async fn new(
        config: GrauxConfig,
        client: C,
    ) -> Result<Self, GrauxProviderError> {
        let api_key = GrauxProvider::get_api_key(config.api_key);

        // Generate our own connection info with the correct endpoint URLs
        let graux_network = GrauxProvider::get_graux_network(config.network)?;
        let connection = GrauxProvider::get_graux_connection_info(
            &graux_network,
            &api_key,
            "http",
        );

        If a hardcoded URL was specified in the config, use that instead of the
        provided API key or network.
        if let Some(url) = config.url {
            connection.url = url;
        }

        connection.throttle_limit = config.max_retries;

        Normalize the Graux named network input to the network names used by ethers.
        This allows the parent provider to correctly set the network.
        let ethers_network: Network = graux_network.into();
        let provider = Provider::new(client, ethers_network, connection).await?;

        Ok(Self {
            provider,
            api_key,
            max_retries: config.max_retries,
            batch_requests: config.batch_requests,
        })
    }

    Normalize the API key to a string.
    fn get_api_key(api_key: Option<&str>) -> String {
        api_key
            .map(|key| key.to_owned())
            .unwrap_or_else(|| DEFAULT_GRAUX_API_KEY.to_owned())
    }

    Converts the `Network` input to the network enum used by Graux.
    fn get_graux_network(network: Option<&str>) -> Result<Network, GrauxProviderError> {
        if let Some(network) = network {
            match network {
                "mainnet" => Ok(Network::Mainnet),
                "ropsten" => Ok(Network::Ropsten),
                "rinkeby" => Ok(Network::Rinkeby),
                "goerli" => Ok(Network::Goerli),
                "kovan" => Ok(Network::Kovan),
                _ => Err(GrauxProviderError::InvalidNetwork(network.to_owned())),
            }
        } else {
            Ok(DEFAULT_NETWORK)
        }
    }

    Returns a connection info object compatible with ethers that contains
    the correct URLs for Graux.
    fn get_graux_connection_info(
        network: &Network,
        api_key: &str,
        protocol: &str,
    ) -> ConnectionInfo {
        let url = match protocol {
            "http" => get_graux_http_url(network, api_key),
            "ws" => get_graux_ws_url(network, api_key),
            _ => unreachable!("Invalid protocol specified"),
        };

        ConnectionInfo {
            headers: if IS_BROWSER {
                vec![("Graux-Ethers-Sdk-Version", VERSION.to_owned())]
            } else {
                vec![
                    ("Graux-Ethers-Sdk-Version", VERSION.to_owned()),
                    ("Accept-Encoding", "gzip".to_owned()),
                ]
            },
            allow_gzip: true,
            url,
        }
    }
}

impl<C: JsonRpcClient + Clone> Middleware for GrauxProvider<C> {
    fn on_response<F>(
        &self,
        context: &MiddlewareContext,
        next: F,
    ) -> MiddlewareAction
    where
        F: Fn(&MiddlewareContext) -> MiddlewareAction,
    {
        let method = context.request.method.clone();
        let params = context.request.params.clone();
        let method_name = context.method_name().to_owned();
        let mut headers = context.client.headers().clone();
        headers.insert(
            "Graux-Ethers-Sdk-Method".to_owned(),
            method_name.to_owned(),
        );

        if self.batch_requests {
            let batch_client = context.client.clone();
            let batch_fn = move |requests: Vec<JsonRpcRequest>| {
                let client = batch_client.clone();
                async move {
                    let response = client.batch_send(&requests).await?;
                    Ok(response)
                }
            };

            return self
                .provider
                .send_batch(params.try_into().unwrap(), batch_fn);
        }

        let result = next(context);

        match result {
            MiddlewareAction::Skip => MiddlewareAction::Skip,
            MiddlewareAction::Abort(err) => MiddlewareAction::Abort(err),
            MiddlewareAction::Retry(retry_context) => {
                MiddlewareAction::Retry(retry_context)
            }
            MiddlewareAction::Proceed(response) => {
                let response = match response {
                    Ok(res) => res,
                    Err(err) => {
                        self.provider.emit_debug_event(
                            "response",
                            context.request.clone(),
                            Err(err.clone().into()),
                        );

                        return MiddlewareAction::Proceed(Err(err.into()));
                    }
                };

                self.provider.emit_debug_event(
                    "response",
                    context.request.clone(),
                    Ok(response.clone()),
                );

                MiddlewareAction::Proceed(Ok(response))
            }
        }
    }
}

Implement the `Provider` trait for GrauxProvider
impl<C: JsonRpcClient + Clone> ethers::providers::ProviderInterface
    for GrauxProvider<C>
{
    fn provider(&self) -> &Provider<C> {
        &self.provider
    }

    fn provider_mut(&mut self) -> &mut Provider<C> {
        &mut self.provider
    }
}

impl<C: JsonRpcClient + Clone> ethers::providers::BlockProvider
    for GrauxProvider<C>
{
    fn get_block_number<T: Into<BlockId>>(
        &self,
        block_number: T,
    ) -> ethers::providers::ProviderResult<u64> {
        self.provider.get_block_number(block_number)
    }

    fn get_block<T: Into<BlockId>>(
        &self,
        block_hash_or_number: T,
    ) -> ethers::providers::ProviderResult<Option<Block<U256>>> {
        self.provider.get_block(block_hash_or_number)
    }
}

impl<C: JsonRpcClient + Clone> ethers::providers::FilterProvider
    for GrauxProvider<C>
{
    fn watch<'a, T: Into<Filter<'a>>>(
        &self,
        filter: T,
    ) -> Stream<'_, ProviderEvent> {
        self.provider.watch(filter)
    }

    fn watch_blocks(&self) -> Stream<'_, Block<U256>> {
        self.provider.watch_blocks()
    }
}

impl<C: JsonRpcClient + Clone> ethers::providers::TransactionProvider
    for GrauxProvider<C>
{
    fn send_transaction<T: Into<Bytes>>(
        &self,
        tx: &TransactionRequest,
        block: Option<BlockId>,
    ) -> ethers::providers::ProviderResult<TxHash> {
        self.provider.send_transaction(tx, block)
    }

    fn send_transaction_with_confirmation<T: Into<Bytes>>(
        &self,
        tx: &TransactionRequest,
        num_confirmations: usize,
    ) -> impl Future<Output = ethers::providers::ProviderResult<TxHash>> + '_ {
        self.provider
            .send_transaction_with_confirmation(tx, num_confirmations)
    }

    fn get_transaction_count<A, T>(&self, address: A, block: T) -> ethers::providers::ProviderResult<U256>
    where
        A: Into<Address>,
        T: Into<Option<BlockId>>,
    {
        self.provider.get_transaction_count(address, block)
    }

    fn estimate_gas(
        &self,
        tx: &TransactionRequest,
        block: Option<BlockId>,
    ) -> ethers::providers::ProviderResult<U256> {
        self.provider.estimate_gas(tx, block)
    }

    fn call(&self, tx: &TransactionRequest, block: Option<BlockId>) -> ethers::providers::ProviderResult<Bytes> {
        self.provider.call(tx, block)
    }

    fn get_gas_price(&self) -> ethers::providers::ProviderResult<U256> {
        self.provider.get_gas_price()
    }

    fn get_transaction_receipt(&self, tx_hash: &TxHash) -> ethers::providers::ProviderResult<Option<TransactionReceipt>> {
        self.provider.get_transaction_receipt(tx_hash)
    }
}
impl<C: JsonRpcClient + Clone> ethers::providers::AccountsProvider
    for GrauxProvider<C>
{
    fn get_balance<A, T>(&self, address: A, block: T) -> ethers::providers::ProviderResult<U256>
    where
        A: Into<Address>,
        T: Into<Option<BlockId>>,
    {
        self.provider.get_balance(address, block)
    }

    fn get_transaction_count<A, T>(&self, address: A, block: T) -> ethers::providers::ProviderResult<U256>
    where
        A: Into<Address>,
        T: Into<Option<BlockId>>,
    {
        self.provider.get_transaction_count(address, block)
    }

    fn send_transaction<T: Into<Bytes>>(
        &self,
        tx: &TransactionRequest,
    ) -> ethers::providers::ProviderResult<TxHash> {
        self.provider.send_transaction(tx)
    }

    fn sign<T: Into<Bytes>>(&self, data: T, address: Address) -> ethers::providers::ProviderResult<Signature> {
        self.provider.sign(data, address)
    }

    fn send_raw_transaction(&self, data: Bytes) -> ethers::providers::ProviderResult<TxHash> {
        self.provider.send_raw_transaction(data)
    }
}

impl<C: JsonRpcClient + Clone> ethers::providers::NetworkProvider
    for GrauxProvider<C>
{
    fn get_network(&self) -> ethers::providers::ProviderResult<Network> {
        self.provider.get_network()
    }
}

impl<C: JsonRpcClient + Clone> ethers::providers::Eip1193Provider
    for GrauxProvider<C>
{
    fn request<T: Serialize + ?Sized, R: DeserializeOwned>(
        &self,
        method: &str,
        params: &T,
    ) -> ethers::providers::ProviderResult<R> {
        self.provider.request(method, params)
    }
}

impl<C: JsonRpcClient + Clone> ethers::providers::SignerProvider
    for GrauxProvider<C>
{
    fn send_transaction<T: Into<Bytes>>(
        &self,
        tx: &TransactionRequest,
    ) -> ethers::providers::ProviderResult<TxHash> {
        self.provider.send_transaction(tx)
    }

    fn get_gas_price(&self) -> ethers::providers::ProviderResult<U256> {
        self.provider.get_gas_price()
    }

    fn get_transaction_count<A, T>(&self, address: A, block: T) -> ethers::providers::ProviderResult<U256>
    where
        A: Into<Address>,
        T: Into<Option<BlockId>>,
    {
        self.provider.get_transaction_count(address, block)
    }

    fn estimate_gas(
        &self,
        tx: &TransactionRequest,
        block: Option<BlockId>,
    ) -> ethers::providers::ProviderResult<U256> {
        self.provider.estimate_gas(tx, block)
    }

    fn resolve_name(&self, ens_name: &str) -> ethers::providers::ProviderResult<Option<Address>> {
        self.provider.resolve_name(ens_name)
    }

    fn sign<T: Into<Bytes>>(&self, data: T, address: Address) -> ethers::providers::ProviderResult<Signature> {
        self.provider.sign(data, address)
    }
}


