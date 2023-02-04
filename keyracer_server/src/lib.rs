use jsonwebtoken::DecodingKey;
use std::collections::HashMap;
use structs::QouteEntry;

pub mod structs;
pub mod utils;

pub mod endpoints {
    pub mod auth;
    pub mod ranked;
    pub mod results;
    pub mod test;
}

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,

    pub words_list: Vec<String>,
    pub quotes_list: Vec<QouteEntry>,
    pub google_jwks: HashMap<String, DecodingKey>,
}
