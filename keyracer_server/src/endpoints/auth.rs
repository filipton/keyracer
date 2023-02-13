use actix_web::{get, post, web, HttpResponse, Responder};
use jsonwebtoken::{Algorithm, Validation};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::str;

use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};

use crate::{structs::GoogleClaims, utils::get_google_certs, AppState};

#[derive(Deserialize, Clone, Debug)]
pub struct JwtHeader {
    pub alg: String,
    pub kid: String,
    pub typ: String,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub created_at: i64,
}

#[post("")]
pub async fn google_auth(data: web::Data<AppState>, token: web::Json<String>) -> impl Responder {
    let token = token.into_inner();

    let header_json = engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD)
        .decode(token.split('.').collect::<Vec<&str>>()[0])
        .unwrap();

    // get_google_certs will throw an error if the certs url is ratelimited (maybe cache this?)
    let header: JwtHeader = serde_json::from_str(str::from_utf8(&header_json).unwrap()).unwrap();
    let google_claims = jsonwebtoken::decode::<GoogleClaims>(
        &token,
        &get_google_certs().await.unwrap()[&header.kid],
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

    let session_id = uuid::Uuid::new_v4().to_string();
    let _ = sqlx::query("INSERT INTO sessions (token, user_id) VALUES ($1, $2)")
        .bind(session_id.clone())
        .bind(user_id)
        .execute(&data.pool)
        .await
        .unwrap();

    return HttpResponse::Ok().json(session_id);
}

#[post("/session")]
pub async fn auth_session(
    data: web::Data<AppState>,
    session_id: web::Json<String>,
) -> impl Responder {
    let session_id = session_id.into_inner();

    let user_info: User = sqlx::query_as(
        "SELECT id, name, email, created_at FROM users 
         INNER JOIN sessions ON users.id = sessions.user_id 
         WHERE sessions.token = $1",
    )
    .bind(session_id)
    .fetch_one(&data.pool)
    .await
    .unwrap();

    return HttpResponse::Ok().json(user_info);
}

#[post("/logout")]
pub async fn logout(data: web::Data<AppState>, session_id: web::Json<String>) -> impl Responder {
    let session_id = session_id.into_inner();

    let _ = sqlx::query("DELETE FROM sessions WHERE token = $1")
        .bind(session_id)
        .execute(&data.pool)
        .await
        .unwrap();

    return HttpResponse::Ok().finish();
}

// this should be /users/{id}
#[get("/{user_id}")]
pub async fn get_user_info(data: web::Data<AppState>, path: web::Path<i64>) -> impl Responder {
    let user_id = path.into_inner();

    let user_info: Result<User, sqlx::Error> = sqlx::query_as(
        "SELECT id, email, name, created_at FROM users 
         WHERE users.id = $1",
    )
    .bind(user_id)
    .fetch_one(&data.pool)
    .await;

    return match user_info {
        Ok(mut user) => {
            user.email = "REDACTED".to_string();
            HttpResponse::Ok().json(user)
        }
        Err(_) => HttpResponse::NotFound().finish(),
    };
}
