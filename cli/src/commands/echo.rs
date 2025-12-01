use crate::{
    core::{errors::rpc_base_error::RPCBaseError, shared_memory::RPC_CLIENT},
    log_error, log_info,
    models::{
        commands::{
            echo_request::{self, EchoRequest},
            echo_response::EchoResponse,
        },
        rpc::{rpc_params::RPCParams, rpc_payload::RPCPayload, rpc_request::RPCRequest},
    },
};
use clap::Args;
use tokio::sync::RwLock;

#[derive(Debug, Args)]
pub struct EchoArgs {
    pub message: String,
}

pub async fn echo(args: EchoArgs) {
    let echo_request = EchoRequest {
        message: args.message,
    };

    print!("{}", &echo_request.message);

    let result = execute(echo_request).await;

    if result.is_err() {
        let error_message = result.unwrap_err();
        log_error!("{}", error_message.reason);
        return;
    }

    log_info!("Echo >> {}", result.unwrap().message);
}

async fn execute(input: EchoRequest) -> Result<EchoResponse, RPCBaseError> {
    let serialized_input = serde_json::to_value(input)
        .map_err(|_| RPCBaseError::build("Object can not serialized"))?;

    let payload = RPCPayload::build(serialized_input).await?;

    let lock = {
        let rpc_client = RPC_CLIENT.read().await;
        let rpc_call_result = rpc_client.call("echo", payload).await?;
        let result = rpc_call_result.result;
        Ok(EchoResponse {
            code: 0,
            message: result,
        })
    }?;

    Ok(lock)
}
