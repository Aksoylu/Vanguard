use rand::seq::SliceRandom;
use sha2::{Digest, Sha256};

pub fn generate_hash(secure_key: String) -> String
{
    let secret = secure_key.to_string() + &generate_salt();

    hash_sha_256(&secret)
}

fn generate_salt() -> String {
    let size = 32;
    let charset: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();
    let mut rng = rand::thread_rng();

    let random_string: String = (0..size)
        .map(|_| *charset.choose(&mut rng).unwrap())
        .collect();

    random_string
}

fn hash_sha_256(input: &str) -> String {
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