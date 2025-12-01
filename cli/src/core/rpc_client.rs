use crate::{
    core::errors::rpc_base_error::RPCBaseError,
    models::{
        base::boot_data::BootData,
        rpc::{
            rpc_params::RPCParams, rpc_payload::RPCPayload, rpc_request::RPCRequest,
            rpc_response::RPCResponse,
        },
    },
};
use reqwest::Client;

pub struct RPCClient {
    pub boot_data: Option<BootData>,
}

impl RPCClient {
    pub fn init(boot_data: BootData) -> RPCClient {
        RPCClient {
            boot_data: Some(boot_data),
        }
    }

    pub async fn call(
        &self,
        method: &str,
        payload: RPCPayload,
    ) -> Result<RPCResponse, RPCBaseError> {
        let rpc_parameter = RPCParams::build(payload).await?;
        let rpc_request = RPCRequest::new(method, rpc_parameter);

        let get_rpc_url = self.get_rpc_url();
        if get_rpc_url.is_none() {
            return Err(RPCBaseError::build("Can not detect RPC target url"));
        }

        let rpc_url = get_rpc_url.unwrap();

        let client = Client::new();

        let response = client
            .post(rpc_url.as_str())
            .json(&rpc_request)
            .send()
            .await
            .map_err(|http_error: reqwest::Error| {
                let error_message = http_error.to_string();
                RPCBaseError::build(error_message.as_str())
            })?;

        if !response.status().is_success() {
            let response_body = response.text().await.map_err(|read_body_error| {
                let response_parse_error_message = format!(
                    "RPC can not read content of response body '{}'",
                    read_body_error.to_string()
                );
                RPCBaseError::build(response_parse_error_message.as_str())
            })?;

            let readed_error_message = format!(
                "Vanguard engine did not sent success signal '{}'",
                response_body
            );
            return Err(RPCBaseError::build(readed_error_message.as_str()));
        }

        let response_body = response.text().await.map_err(|read_body_error| {
            let error_message = format!(
                "Reading body failure on vanguard engine response: {}",
                read_body_error.to_string()
            );
            RPCBaseError::build(error_message.as_str())
        })?;

        println!("{}", &response_body);

        Ok(RPCResponse {
            jsonrpc: "jsonrpc".to_string(),
            result: response_body,
            id: 21,
        })
    }

    fn get_rpc_url(&self) -> Option<String> {
        if self.boot_data.is_none() {
            return None;
        }

        let get_rpc_session = self
            .boot_data
            .as_ref()
            .and_then(|boot_data| boot_data.rpc_session.as_ref())
            .map(|rpc_session_info| rpc_session_info.clone());

        if get_rpc_session.is_none() {
            return None;
        }

        let rpc_session = get_rpc_session.unwrap();

        let rpc_url = format!("http://{}:{}", &rpc_session.ip_addr, &rpc_session.port);

        Some(rpc_url)
    }
}

impl Default for RPCClient {
    fn default() -> Self {
        Self { boot_data: None }
    }
}
