use crate::{endpoints::auth::User, structs::GoogleCerts};
use actix_web::HttpRequest;
use jsonwebtoken::DecodingKey;
use sqlx::PgPool;
use std::collections::HashMap;

pub async fn get_google_certs() -> Result<HashMap<String, DecodingKey>, ()> {
    let client = awc::Client::default();

    let res: GoogleCerts =
        serde_json::from_str(std::fs::read_to_string("./certs.json").unwrap().as_str()).unwrap();

    let mut google_jwks: HashMap<String, DecodingKey> = HashMap::new();
    for key in res.keys {
        let d_key = DecodingKey::from_rsa_components(&key.n, &key.e).unwrap();
        google_jwks.insert(key.kid, d_key);
    }

    return Ok(google_jwks);
}

pub async fn verify_token(req: &HttpRequest, pool: &PgPool) -> Option<User> {
    let token = match req.headers().get("Auth") {
        Some(header) => header.to_str().unwrap(),
        None => return None,
    };

    let user = sqlx::query_as(
        "SELECT * FROM users INNER JOIN sessions ON users.id = sessions.user_id 
         WHERE sessions.token = $1",
    )
    .bind(token)
    .fetch_one(pool)
    .await;

    return match user {
        Ok(user) => Some(user),
        Err(_) => None,
    };
}
