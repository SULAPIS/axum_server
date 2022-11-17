use anyhow::{anyhow, Ok, Result};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    number: String,
    password: String,
}

pub fn generate_token(number: String, password: String) -> Result<String> {
    let my_claims = Claims { number, password };
    let key = b"secret";
    let header = Header {
        kid: Some("signing_key".to_owned()),
        alg: Algorithm::HS512,
        ..Default::default()
    };
    encode(&header, &my_claims, &EncodingKey::from_secret(key)).or(Err(anyhow!("generate failed")))
}

#[test]
fn test_generate_token() {
    println!(
        "{:?}",
        generate_token("12345678901".to_string(), "123456".to_string())
    );
}
