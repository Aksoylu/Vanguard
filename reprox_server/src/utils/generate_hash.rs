use super::sha_256::sha_256;


pub fn generate_hash(secure_key: String, salt: String) -> String
{
    let secret = secure_key.to_string() + &salt;

    sha_256(&secret)
}