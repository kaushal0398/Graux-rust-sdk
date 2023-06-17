use ethers::{
    providers::{Middleware, Provider},
    types::{BlockNumber, DebugTransaction, SimulateAssetChangesResponse, SimulateExecutionResponse, TransactionReceipt, TransactionRequest, TransactionResponse},
    utils::hexlify,
    Middleware as _,
};
use std::convert::TryFrom;

pub struct Graux {
    provider: Provider,
}

impl Graux {
    pub fn new(provider: Provider) -> Self {
        Self { provider }
    }

    pub async fn send_private_transaction(
        &self,
        signed_transaction: String,
        max_block_number: Option<u64>,
        options: Option<SendPrivateTransactionOptions>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let hex_block_number = max_block_number.map(hexlify);
        let tx = json!({
            "tx": signed_transaction,
            "maxBlockNumber": hex_block_number,
            "preferences": options,
        });
        let response = self
            .provider
            .send("eth_sendPrivateTransaction", vec![tx])
            .await?;
        Ok(response[0].as_str().unwrap().to_owned())
    }

    pub async fn cancel_private_transaction(
        &self,
        transaction_hash: String,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let tx = json!({
            "txHash": transaction_hash,
        });
        let response = self
            .provider
            .send("eth_cancelPrivateTransaction", vec![tx])
            .await?;
        Ok(response[0].as_bool().unwrap())
    }

    pub async fn simulate_asset_changes_bundle(
        &self,
        transactions: Vec<DebugTransaction>,
        block_identifier: Option<BlockIdentifier>,
    ) -> Result<Vec<SimulateAssetChangesResponse>, Box<dyn std::error::Error>> {
        let params = match block_identifier {
            Some(block) => vec![transactions, block],
            None => vec![transactions],
        };
        let response = self
            .provider
            .send("graux_simulateAssetChangesBundle", params)
            .await?;
        Ok(response.into_iter().map(TryFrom::try_from).collect::<Result<_, _>>()?)
    }

    pub async fn simulate_asset_changes(
        &self,
        transaction: DebugTransaction,
        block_identifier: Option<BlockIdentifier>,
    ) -> Result<SimulateAssetChangesResponse, Box<dyn std::error::Error>> {
        let params = match block_identifier {
            Some(block) => vec![transaction, block],
            None => vec![transaction],
        };
        let response = self
            .provider
            .send("graux_simulateAssetChanges", params)
            .await?;
        Ok(TryFrom::try_from(response)?)
    }

    pub async fn simulate_execution_bundle(
        &self,
        transactions: Vec<DebugTransaction>,
        block_identifier: Option<BlockIdentifier>,
    ) -> Result<Vec<SimulateExecutionResponse>, Box<dyn std::error::Error>> {
        let params = match block_identifier {
            Some(block) => vec![transactions, block],
            None => vec![transactions],
        };
        let response = self
            .provider
            .send("graux_simulateExecutionBundle", params)
            .await?;
        Ok(response.into_iter().map(TryFrom::try_from).collect::<Result<_, _>>()?)
    }

    pub async fn simulate_execution(
        &self,
        transaction: DebugTransaction,
        block_identifier: Option<BlockIdentifier>,
    ) -> Result<SimulateExecutionResponse, Box<dyn std::error::Error>> {
        let params = match block_identifier {
            Some(block) => vec![transaction, block],
            None => vec![transaction],
        };
        let response = self
            .provider
            .send("graux_simulateExecution", params)
            .await?;
        Ok(TryFrom::try_from(response)?)
    }

    pub async fn get_private_transaction_receipt(
        &self,
        transaction_hash: String,
    ) -> Result<Option<TransactionReceipt>, Box<dyn std::error::Error>> {
        let tx = json!({
            "txHash": transaction_hash,
        });
        let response = self
            .provider
            .send("eth_getPrivateTransactionReceipt", vec![tx])
            .await?;
        match response[0].as_object() {
            Some(obj) if obj.is_empty() => Ok(None),
            Some(obj) => Ok(Some(TryFrom::try_from(obj.clone())?)),
            None => Ok(None),
        }
    }
}

#[derive(Debug, Serialize)]
struct SendPrivateTransactionOptions {
    gas: Option<u64>,
    gas_price: Option<u64>,
    value: Option<u64>,
    max_priority_fee_per_gas: Option<u64>,
    max_fee_per_gas: Option<u64>,
}

#[derive(Debug, Serialize)]
struct BlockIdentifier {
    block_hash: Option<String>,
    block_number: Option<String>,
}
