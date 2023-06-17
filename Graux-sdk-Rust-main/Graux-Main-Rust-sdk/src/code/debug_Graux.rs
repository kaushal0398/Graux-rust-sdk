use crate::types::{BlockIdentifier, DebugCallTrace, DebugCallTracer, DebugPrestateTrace, DebugPrestateTracer, DebugTransaction};
use crate::graux_config::GrauxConfig;
use crate::utils::{hex_strip_zeros, hex_value, is_hex_string};

DebugNamespace contains methods to access the non-standard RPC methods for inspecting and debugging transactions.
pub struct DebugNamespace {
    config: GrauxConfig,
}

impl DebugNamespace {
    // Constructor
    pub fn new(config: GrauxConfig) -> Self {
        DebugNamespace { config }
    }

    Runs an `eth_call` with the context of the provided block execution using the final state of the parent block as the base.
    pub async fn trace_call(&self, transaction: DebugTransaction, block_identifier: BlockIdentifier, tracer: DebugCallTracer) -> Result<DebugCallTrace, Box<dyn std::error::Error>> {
        let provider = self.config.get_provider().await?;
        let params = [transaction, block_identifier, parse_tracer_params(tracer)];
        let result = provider.send("debug_traceCall", &params).await?;
        
    }

    pub async fn trace_prestate(&self, transaction: DebugTransaction, block_identifier: BlockIdentifier, tracer: DebugPrestateTracer) -> Result<DebugPrestateTrace, Box<dyn std::error::Error>> {
        let provider = self.config.get_provider().await?;
        let params = [transaction, block_identifier, parse_tracer_params(tracer)];
        let result = provider.send("debug_traceCall", &params).await?;
        // Parse and return the result
        // ...
    }

    pub async fn trace_transaction(&self, transaction_hash: String, tracer: DebugCallTracer, timeout: Option<String>) -> Result<DebugCallTrace, Box<dyn std::error::Error>> {
        let provider = self.config.get_provider().await?;
        let params = [transaction_hash, parse_tracer_params(tracer, timeout)];
        let result = provider.send("debug_traceTransaction", &params).await?;
        Parse and return the result
    }

    Replays a block that has already been mined.
    pub async fn trace_block(&self, block_identifier: BlockIdentifier, tracer: DebugCallTracer) -> Result<DebugCallTrace, Box<dyn std::error::Error>> {
        let provider = self.config.get_provider().await?;
        let (method, params) = if is_hex_string(&block_identifier, 32) {
            (
                "debug_traceBlockByHash",
                [block_identifier, parse_tracer_params(tracer)],
            )
        } else {
            (
                "debug_traceBlockByNumber",
                [
                    hex_strip_zeros(hex_value(block_identifier)),
                    parse_tracer_params(tracer),
                ],
            )
        };
        let result = provider.send(method, &params).await?;
        Parse and return the result
    }
}

RawTracer represents the raw parameters for the tracer.
struct RawTracer {
    tracer: String,
    tracer_config: Option<TracerConfig>,
}

TracerConfig represents the configuration options for the tracer.
struct TracerConfig {
    only_top_call: Option<bool>,
    timeout: Option<String>,
}

Parses the tracer parameters into a RawTracer object.
fn parse_tracer_params(tracer: DebugCallTracer, timeout: Option<String>) -> RawTracer {
    let tracer_config = TracerConfig {
        only_top_call: tracer.only_top_call,
        timeout,
    };

    RawTracer {
        tracer: tracer.type,
        tracer_config: Some(tracer_config),
    }
}
