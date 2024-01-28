use sha2::{Digest, Sha256};

pub fn sha_256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();

    // Convert the hash result to a hexadecimal string
    let hash_hex_string: String = result
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect();

    hash_hex_string
}