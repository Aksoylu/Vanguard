use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use base64::engine::general_purpose;
use base64::{decode, encode};
use rand::seq::SliceRandom;
use rand::RngCore;
use sha2::{Digest, Sha256};
use std::fmt::Write;

pub fn generate_hash(secure_key: String) -> String {
    let secret = secure_key.to_string() + &generate_salt();

    hash_sha_256(&secret)
}

fn generate_salt() -> String {
    let size = 32;
    let charset: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        .chars()
        .collect();
    let mut rng = rand::thread_rng();

    let random_string: String = (0..size)
        .map(|_| *charset.choose(&mut rng).unwrap())
        .collect();

    random_string
}

pub fn hash_sha_256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();

    let mut hash_hex_string = String::new(); // Her bayt 2 karaktere dönüşeceği için kapasiteyi ayarla

    for byte in result {
        write!(&mut hash_hex_string, "{:02x}", byte).expect("Unable to write");
    }

    hash_hex_string
}

pub fn generate_nonce() -> [u8; 12] {
    let mut nonce = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce);
    nonce
}

pub fn encrypt_aes256(encryption_key: &str, plaintext: &str, nonce: &[u8; 12]) -> (String, String) {
    let key_bytes = parse_string_as_aes_key(encryption_key);
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce);
    
    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .expect("encryption failed");
    (encode(ciphertext), encode(nonce))
}

pub fn decrypt_aes256(
    decryption_key: &str,
    ciphertext_b64: &str,
    nonce_b64: &str,
) -> Option<String> {
    let nonce_bytes: Vec<u8> = decode(nonce_b64).ok()?;

    let key_bytes = parse_string_as_aes_key(decryption_key);
    let parsed_key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(parsed_key);

    let ciphertext_bytes = decode(ciphertext_b64).ok()?;
    let nonce = Nonce::from_slice(&nonce_bytes);
    let plaintext = cipher.decrypt(nonce, ciphertext_bytes.as_ref()).ok()?;
    String::from_utf8(plaintext).ok()
}

pub fn parse_string_as_aes_key(key_str: &str) -> [u8; 32] {
    let mut key_bytes = [0u8; 32];
    let bytes = key_str.as_bytes();
    for (i, b) in bytes.iter().enumerate().take(32) {
        key_bytes[i] = *b;
    }
    key_bytes
}
