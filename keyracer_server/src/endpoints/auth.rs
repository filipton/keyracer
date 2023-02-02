use actix_web::{get, post, web, HttpResponse, Responder};
use jsonwebtoken::{Algorithm, Validation};
use serde::Deserialize;
use sqlx::Row;
use std::str;

use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};

use crate::{structs::GoogleClaims, AppState};

#[derive(Deserialize, Clone, Debug)]
pub struct JwtHeader {
    pub alg: String,
    pub kid: String,
    pub typ: String,
}

#[post("")]
pub async fn google_auth(data: web::Data<AppState>, token: web::Json<String>) -> impl Responder {
    let token = token.into_inner();

    let header_json = engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD)
        .decode(token.split('.').collect::<Vec<&str>>()[0])
        .unwrap();

    let header: JwtHeader = serde_json::from_str(str::from_utf8(&header_json).unwrap()).unwrap();
    let google_claims = jsonwebtoken::decode::<GoogleClaims>(
        &token,
        &data.google_jwks[&header.kid],
        &Validation::new(Algorithm::RS256),
    )
    .unwrap()
    .claims;

    let user_id_query = sqlx::query("SELECT id FROM users WHERE email = $1")
        .bind(google_claims.email.clone())
        .fetch_one(&data.pool)
        .await;

    let user_id = match user_id_query {
        Ok(row) => row.get::<i64, _>(0),
        Err(_) => {
            let user_id = idgenerator::IdInstance::next_id();

            let _ = sqlx::query("INSERT INTO users (id, name, email) VALUES ($1, $2, $3)")
                .bind(user_id)
                .bind(google_claims.name.clone())
                .bind(google_claims.email.clone())
                .execute(&data.pool)
                .await
                .unwrap();

            user_id
        }
    };

    return HttpResponse::Ok().body(format!("{:?}", user_id));
}
