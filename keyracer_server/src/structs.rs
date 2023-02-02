use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QouteEntry {
    pub quote: String,
    pub author: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyracerResponse {
    pub time: i64,
    pub chars_written: i32,
    pub chars_correct: i32,
    pub chars_in_correct_words: i32,
    pub history: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyracerData {
    pub time: i64,
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
