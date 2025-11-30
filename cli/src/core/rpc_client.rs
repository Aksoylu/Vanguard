use reqwest::Client;
use crate::{core::{errors::rpc_call_error::RPCCallError, shared_memory::RPC_CLIENT}, models::{base::boot_data::BootData, rpc::{rpc_params::RPCParams, rpc_payload::RPCPayload, rpc_request::RPCRequest, rpc_session}}};
use serde_json::Value;

// todo: development required here
pub struct RPCClient{
    pub boot_data: Option<BootData>
}

impl RPCClient{
    pub fn init(boot_data: BootData) -> RPCClient{

        RPCClient{boot_data: Some(boot_data)}
    }

    pub async fn send(&self, method: &str, payload: RPCPayload)-> Result<RPCResponse,RPCCallError>{
        let rpc_parameter = RPCParams::new(payload)?;
        let rpc_request = RPCRequest::new(method, rpc_parameter);
        

        let rpc_url = self.get_rpc_url()?;

        let client = Client::new();

        let response = client
            .post(rpc_url.as_str())
            .json(&rpc_request)
            .send()
            .await?;

        if response.status().is_success() {
            // Cevabı doğrudan RPCResponse struct'ına deserialize etme
            let response_body = response.json().await?;
            println!("✅ Başarılı Cevap: {:#?}", response_body);
        } else {
            eprintln!("❌ Hata Durumu: {}", response.status());
            let body = response.text().await?;
            eprintln!("Cevap Gövdesi: {}", body);
        }

        RPCResponse{}
    }

    fn get_rpc_url(&self) -> Option<String> {
        if self.boot_data.is_none(){
            return None;
        }

        let get_rpc_session = 
            self.boot_data.as_ref()
            .and_then(|boot_data| boot_data.rpc_session.as_ref()) 
            .map(|rpc_session_info| rpc_session_info.clone());  

        if get_rpc_session.is_none(){
            return None;
        }      

        let rpc_session = get_rpc_session.unwrap();


        let rpc_url = format!("http://{}:{}", 
            &rpc_session.ip_addr,
            &rpc_session.port
        );

        Some(rpc_url)
    }


}

impl Default for RPCClient{
    fn default() -> Self {
        Self{boot_data: None}
    }
}