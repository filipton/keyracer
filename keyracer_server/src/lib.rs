use jsonwebtoken::DecodingKey;
use std::collections::HashMap;

pub mod structs;
pub mod utils;

pub mod endpoints {
    pub mod auth;
    pub mod data;
    pub mod ranked;
    pub mod results;
}

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,

    pub words_list: String,
    pub quotes_list: String,
    pub google_jwks: HashMap<String, DecodingKey>,
}
