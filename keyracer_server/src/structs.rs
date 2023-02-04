use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QouteEntry {
    pub quote: String,
    pub author: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyracerResponse {
    pub time: i32,
    pub chars_written: i32,
    pub chars_correct: i32,
    pub chars_in_correct_words: i32,
    pub history: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RankedResponse {
    pub time: i32,
    pub quote_id: i32,
    pub chars_written: i32,
    pub chars_correct: i32,
    pub chars_in_correct_words: i32,
    pub history: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyracerData {
    pub time: i32,
    pub chars_written: i32,
    pub chars_correct: i32,
    pub chars_in_correct_words: i32,
    pub history: Vec<HistoryEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HistoryEntry {
    pub time: i32,
    pub input: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GoogleClaims {
    pub email: String,
    pub name: String,
    pub picture: String,
    pub given_name: String,
    pub exp: i64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GoogleCerts {
    pub keys: Vec<GoogleCert>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GoogleCert {
    pub alg: String,
    pub e: String,
    pub kid: String,
    pub kty: String,
    pub n: String,
}

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct NrResult {
    pub id: i32,
    pub user_id: i64,
    pub time: i32,
    pub wpm: f64,
    pub acc: f64,
    pub max_ks: i32,
    pub created_at: i64,
}
