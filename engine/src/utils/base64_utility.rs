use base64::{engine::general_purpose, Engine as _};

pub fn decode_b64(input: String) -> Option<String> {
    let decoded_bytes = general_purpose::STANDARD.decode(input);
    if decoded_bytes.is_err() {
        return None;
    }

    let decoded_str = String::from_utf8(decoded_bytes.unwrap());

    if decoded_str.is_err() {
        return None;
    }

    Some(decoded_str.unwrap())
}
