use crate::structs::GoogleCerts;
use jsonwebtoken::DecodingKey;
use std::collections::HashMap;

pub async fn get_google_certs() -> Result<HashMap<String, DecodingKey>, ()> {
    let client = awc::Client::default();

    let req = client.get("https://www.googleapis.com/oauth2/v3/certs");
    let res = req
        .send()
        .await
        .unwrap()
        .json::<GoogleCerts>()
        .await
        .unwrap();

    let mut google_jwks: HashMap<String, DecodingKey> = HashMap::new();
    for key in res.keys {
        let d_key = DecodingKey::from_rsa_components(&key.n, &key.e).unwrap();
        google_jwks.insert(key.kid, d_key);
    }

    return Ok(google_jwks);
}
