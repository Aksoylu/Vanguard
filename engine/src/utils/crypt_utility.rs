use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use rand::seq::SliceRandom;
use sha2::{Digest, Sha256};

pub fn generate_hash(secure_key: String) -> String {
    let secret = secure_key.to_string() + &generate_salt(128);

    hash_sha_256(&secret)
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

pub fn decrypt_aes256_hex(key_hex: &str, ciphertext_hex: &str, nonce_hex: &str) -> Option<String> {
    println!("key_hex {}", key_hex);
    println!("ciphertext_hex {}", ciphertext_hex);
    println!("nonce_hex {}", nonce_hex);


    let key_bytes = hex::decode(key_hex).ok()?;
    let nonce_bytes = hex::decode(nonce_hex).ok()?;

    println!(
        "Key bytes length: {}, Nonce bytes length: {}",
        key_bytes.len(),
        nonce_bytes.len()
    );

    let ciphertext_bytes: Vec<u8> =  hex::decode(ciphertext_hex).ok()?;

    if key_bytes.len() != 32 || nonce_bytes.len() != 12 {
        return None;
    }

    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let plaintext_bytes = cipher.decrypt(nonce, ciphertext_bytes.as_ref()).ok()?;
    String::from_utf8(plaintext_bytes).ok()
}
