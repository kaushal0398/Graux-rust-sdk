use ethers_core::types::{Address, Block, BlockTag, Log, TransactionReceipt, TransactionRequest, TransactionResponse};
use ethers_core::utils::{BigEndianHash, to_32bytes, to_64bytes};
use ethers_providers::{Middleware, MiddlewareError, Provider};
use std::convert::TryFrom;

custom error type for the GrauxConfig
#[derive(Debug)]
enum GrauxError {
    ConfigError(String),
}

the GrauxConfig struct
struct GrauxConfig {
    provider: Provider,
}

impl GrauxConfig {
    fn new() -> Result<Self, GrauxError> {
        // Initialize the Ethereum provider
        let provider = Provider::try_from("https://api.example.com")?;

        Ok(Self { provider })
    }

    fn get_provider(&self) -> &Provider {
        &self.provider
    }
}
struct GrauxCoreNamespace {
    config: GrauxConfig,
}

{
    fn new(config: GrauxConfig) -> Self {
        Self { config }
    }

    async fn get_balance(
        &self,
        address_or_name: &str,
        block_tag: Option<BlockTag>,
    ) -> Result<BigEndianHash, MiddlewareError> {
        let provider = self.config.get_provider();
        let address = Address::from(address_or_name);

        provider.get_balance(address, block_tag).await
    }

    async fn get_code(
        &self,
        address_or_name: &str,
        block_tag: Option<BlockTag>,
    ) -> Result<Vec<u8>, MiddlewareError> {
        let provider = self.config.get_provider();
        let address = Address::from(address_or_name);

        provider.get_code(address, block_tag).await
    }

    async fn get_storage_at(
        &self,
        address_or_name: &str,
        position: BigEndianHash,
        block_tag: Option<BlockTag>,
    ) -> Result<Vec<u8>, MiddlewareError> {
        let provider = self.config.get_provider();
        let address = Address::from(address_or_name);

        provider.get_storage_at(address, position, block_tag).await
    }

    async fn get_transaction_count(
        &self,
        address_or_name: &str,
        block_tag: Option<BlockTag>,
    ) -> Result<u64, MiddlewareError> {
        let provider = self.config.get_provider();
        let address = Address::from(address_or_name);

        provider.get_transaction_count(address, block_tag).await
    }

    async fn get_block(
        &self,
        block_hash_or_block_tag: BlockTag,
    ) -> Result<Block, MiddlewareError> {
        let provider = self.config.get_provider();

        provider.get_block(block_hash_or_block_tag).await
    }

    async fn get_block_with_transactions(
        &self,
        block_hash_or_block_tag: BlockTag,
    ) -> Result<Block, MiddlewareError> {
        let provider = self.config.get_provider();

        provider.get_block_with_txs(block_hash_or_block_tag).await
    }

    async fn get_network(&self) -> Result<String, MiddlewareError> {
        let provider = self.config.get_provider();

        provider.get_network().await
    }

    async fn get_block_number(&self) -> Result<u64, MiddlewareError> {
        let provider = self.config.get_provider();

        provider.get_block_number().await
    }

    async fn get_gas_price(&self) -> Result<BigEndianHash, MiddlewareError> {
        let provider = self.config.get_provider();

        provider.get_gas_price().await
    }

    async fn get_fee_data(&self) -> Result<BigEndianHash, MiddlewareError> {
        let provider = self.config.get_provider();

        provider.get_fee_data().await
    }

    async fn ready(&self) -> Result<(), MiddlewareError> {
        let provider = self.config.get_provider();

        provider.ready().await
    }

    async fn call(
        &self,
        tx: TransactionRequest,
        block_tag: Option<BlockTag>,
    ) -> Result<Vec<u8>, MiddlewareError> {
        let provider = self.config.get_provider();

        provider.call(tx, block_tag).await
    }

    async fn estimate_gas(
        &self,
        tx: TransactionRequest,
    ) -> Result<BigEndianHash, MiddlewareError> {
        let provider = self.config.get_provider();

        provider.estimate_gas(tx).await
    }

    async fn get_transaction(
        &self,
        transaction_hash: BigEndianHash,
    ) -> Result<TransactionResponse, MiddlewareError> {
        let provider = self.config.get_provider();

        provider.get_transaction(transaction_hash).await
    }

    async fn get_transaction_receipt(
        &self,
        transaction_hash: BigEndianHash,
    ) -> Result<TransactionReceipt, MiddlewareError> {
        let provider = self.config.get_provider();

        provider.get_transaction_receipt(transaction_hash).await
    }

    async fn send_transaction(
        &self,
        tx: TransactionRequest,
    ) -> Result<BigEndianHash, MiddlewareError> {
        let provider = self.config.get_provider();

        provider.send_transaction(tx).await
    }

    async fn wait_for_transaction(
        &self,
        transaction_hash: BigEndianHash,
    ) -> Result<TransactionReceipt, MiddlewareError> {
        let provider = self.config.get_provider();

        provider.wait_for_transaction(transaction_hash).await
    }

     async fn get_logs(
        &self,
        filter: impl Into<LogFilter>,
    ) -> Result<Vec<Log>, MiddlewareError> {
        let provider = self.config.get_provider();

        provider.get_logs(filter).await
    }

    async fn send(&self, method: &str, params: Vec<serde_json::Value>) -> Result<serde_json::Value, MiddlewareError> {
        let provider = self.config.get_provider();

        provider.send(method, params).await
    }

    async fn find_contract_deployer(
        &self,
        contract_address: &str,
        from_block: Option<BlockTag>,
        to_block: Option<BlockTag>,
    ) -> Result<Option<(Address, u64)>, MiddlewareError> {
        let provider = self.config.get_provider();
        let address = Address::from(contract_address);

        provider
            .find_contract_deployer(address, from_block, to_block)
            .await
    }
}

fn main() {
    // Initialize GrauxConfig
    let config = match GrauxConfig::new() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Failed to initialize GrauxConfig: {:?}", err);
            return;
        }
    };

    Initialize GrauxCoreNamespace
    let graux = GrauxCoreNamespace::new(config);

    Use the GrauxCoreNamespace methods
    async {
        let balance = graux.get_balance("0x1234567890", None).await;
        println!("Balance: {:?}", balance);

        let code = graux.get_code("0x1234567890", None).await;
        println!("Code: {:?}", code);
    };
}


Replace "https://api.example.com" with the actual URL of Ethereum provider.
