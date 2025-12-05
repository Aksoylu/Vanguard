use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use rand::{seq::SliceRandom, thread_rng, RngCore};
use sha2::{Digest, Sha256};

/// Generates a 12-byte (96-bit) random Nonce for AES-GCM
/// and returns it as a Hex (hexadecimal) string.
pub fn generate_nonce_hex() -> String {
    let mut nonce = [0u8; 12];

    thread_rng().fill_bytes(&mut nonce);

    hex::encode(nonce)
}

/// Generates a random salt of the specified length in bytes
/// and returns it as a Hex (hexadecimal) string.
pub fn generate_hash(secure_key: String) -> String {
    let secret = secure_key.to_string() + &generate_salt(128);

    hash_sha_256(&secret)
}

/// Hashes the input string using SHA-256 and returns the hash
/// as a hexadecimal string.
pub fn hash_sha_256(input: &str) -> String {
    let digest = Sha256::digest(input.as_bytes());
    let mut key = [0u8; 32];
    key.copy_from_slice(&digest);
    let key_as_str = key
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>();

    key_as_str
}

/// Encrypts the given plaintext using AES-256-GCM with the provided
/// key and nonce (both in hexadecimal string format).
pub fn encrypt_aes256_hex(key_hex: &str, plaintext: &str, nonce_hex: &str) -> Option<String> {
    let key_bytes = hex::decode(key_hex).ok()?;
    let nonce_bytes = hex::decode(nonce_hex).ok()?;

    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, plaintext.as_bytes()).ok()?;

    return Some(hex::encode(ciphertext));
}

fn generate_salt(size: i32) -> String {
    let charset: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        .chars()
        .collect();
    let mut rng = rand::thread_rng();

    let random_string: String = (0..size)
        .map(|_| *charset.choose(&mut rng).unwrap())
        .collect();

    random_string
}
