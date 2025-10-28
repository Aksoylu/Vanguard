use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};
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

pub fn encrypt_aes256(key_bytes: &[u8; 32], plaintext: &str, nonce: &[u8; 12]) -> (String, String) {
    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce);
    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .expect("encryption failed");
    (encode(ciphertext), encode(nonce))
}

pub fn decrypt_aes256(
    key_bytes: &[u8; 32],
    ciphertext_b64: &str,
    nonce_b64: &str,
) -> Option<String> {
    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce_bytes = decode(nonce_b64).ok()?;
    let ciphertext_bytes = decode(ciphertext_b64).ok()?;
    let nonce = Nonce::from_slice(&nonce_bytes);
    let plaintext = cipher.decrypt(nonce, ciphertext_bytes.as_ref()).ok()?;
    String::from_utf8(plaintext).ok()
}
