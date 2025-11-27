pub struct RpcParams{

}

pub struct JrpcPayload {
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<serde_json::Value>,
    pub id: u64,
}

impl JrpcPayload {
    pub fn new(method: &str, params: Vec<serde_json::Value>, id: u64) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params,
            id,
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "jsonrpc": self.jsonrpc,
            "method": self.method,
            "params": self.params,
            "id": self.id,
        })
    }

    pub fn pack(&self) {
        let key = Key::from_slice(b"an_example_32_byte_long_secretkey!!");
        let cipher = ChaCha20Poly1305::new(key);
        let nonce_bytes = b"unique_nonce12"; // 12 byte
        let nonce = Nonce::from_slice(nonce_bytes);

        // 2. JSON-RPC payload (plaintext)
        let rpc_plain = json!({
            "method": "add_route",
            "params": {
                "route": "/api/test",
                "target": "http://127.0.0.1:4242"
            },
            "id": 1
        });
        let plaintext = serde_json::to_vec(&rpc_plain)?;

        // 3. AEAD encrypt
        let ciphertext = cipher.encrypt(nonce, plaintext.as_ref())?;
        let payload_b64 = encode(&ciphertext);
        let nonce_b64 = encode(nonce_bytes);

        // 4. JSON-RPC wrapper
        let rpc_encrypted = json!({
            "jsonrpc": "2.0",
            "method": "secure_method",
            "params": {
                "payload": payload_b64,
                "nonce": nonce_b64
            },
            "id": 1
        });
    }
}
