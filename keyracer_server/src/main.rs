use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use std::str;

use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use std::collections::HashMap;

use actix_web::{
    get, post,
    web::{self, scope, Bytes},
    App, HttpResponse, HttpServer, Responder,
};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct AppState {
    pub words_list: Vec<String>,
    pub quotes_list: Vec<QouteEntry>,
    pub google_jwks: HashMap<String, DecodingKey>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QouteEntry {
    pub quote: String,
    pub author: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyracerResponse {
    time: i64,
    chars_written: i32,
    chars_correct: i32,
    chars_in_correct_words: i32,
    history: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyracerData {
    time: i64,
    chars_written: i32,
    chars_correct: i32,
    chars_in_correct_words: i32,
    history: Vec<HistoryEntry>,
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let google_keys = get_google_certs().await.unwrap();

    let mut port = 8080;
    match std::env::var("PORT") {
        Ok(val) => port = val.parse::<u16>().unwrap(),
        Err(_) => println!("PORT env not specified, using default"),
    }

    println!("Starting server on port: {}", port);

    HttpServer::new(move || {
        let app_state: AppState = AppState {
            words_list: std::fs::read_to_string("./words_list.txt")
                .unwrap()
                .lines()
                .filter_map(|x| {
                    //if x.len() > 10 {
                    //    return None;
                    //}
                    return Some(x.to_string());
                })
                .collect(),
            quotes_list: serde_json::from_str(
                std::fs::read_to_string("./quotes.json").unwrap().as_str(),
            )
            .unwrap(),
            google_jwks: google_keys.clone(),
        };

        let cors = actix_cors::Cors::default()
            .allow_any_origin()
            .send_wildcard()
            .allow_any_method()
            .allow_any_header()
            .max_age(None);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(app_state.clone()))
            .service(
                scope("")
                    .service(get_index)
                    .service(get_test)
                    .service(test)
                    .service(get_quotes_entry)
                    .service(test_google_auth)
                    .service(post_keyracer_response),
            )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

async fn get_google_certs() -> Result<HashMap<String, DecodingKey>, ()> {
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

#[get("/")]
async fn get_index() -> impl Responder {
    return HttpResponse::Ok().body("Nothing here yet!");
}

#[get("/words/{count}")]
async fn get_test(data: web::Data<AppState>, path: web::Path<i64>) -> impl Responder {
    let mut rng = rand::thread_rng();

    let random_words: Vec<String> = (1..=path.into_inner())
        .into_iter()
        .map(|_| {
            let index = rng.gen_range(0..data.words_list.len());
            data.words_list[index].to_string()
        })
        .collect();

    return HttpResponse::Ok().json(random_words);
}

#[get("/quote")]
async fn get_quotes_entry(data: web::Data<AppState>) -> impl Responder {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..data.quotes_list.len());

    return HttpResponse::Ok().json(data.quotes_list[index].clone());
}

#[get("/test")]
async fn test() -> impl Responder {
    return HttpResponse::Ok()
        .body(std::fs::read_to_string("/home/notpilif/Downloads/test.html").unwrap());
}

#[post("/response")]
async fn post_keyracer_response(response_data: web::Json<KeyracerResponse>) -> impl Responder {
    let data: KeyracerData = KeyracerData {
        time: response_data.time,
        chars_written: response_data.chars_written,
        chars_correct: response_data.chars_correct,
        chars_in_correct_words: response_data.chars_in_correct_words,
        history: response_data
            .history
            .lines()
            .map(|x| {
                let splitted: Vec<&str> = x.split("><").collect();

                return HistoryEntry {
                    time: splitted[0].to_string().parse().unwrap(),
                    input: splitted[1].to_string(),
                };
            })
            .collect(),
    };

    let wpm_time = response_data.time as f64 / 60000f64;
    let wpm = response_data.chars_in_correct_words as f64 / 5f64 / wpm_time;

    let time = format!("{:.2}s", response_data.time as f64 / 10f64 / 100f64);
    let accuracy = format!(
        "{:.2}%",
        response_data.chars_correct as f64 / response_data.chars_written as f64 * 100f64
    );

    println!("WPM: {:.2}  TIME: {}  ACC: {}", wpm, time, accuracy);

    return HttpResponse::Ok().body("");
}

#[derive(Deserialize, Clone, Debug)]
pub struct JwtHeader {
    pub alg: String,
    pub kid: String,
    pub typ: String,
}

#[post("/auth")]
async fn test_google_auth(
    data: web::Data<AppState>,
    jwt_token: web::Json<String>,
) -> impl Responder {
    let header_json = engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD)
        .decode(jwt_token.0.split('.').collect::<Vec<&str>>()[0])
        .unwrap();

    let header: JwtHeader = serde_json::from_str(str::from_utf8(&header_json).unwrap()).unwrap();
    let test_token = jsonwebtoken::decode::<GoogleClaims>(
        &jwt_token,
        &data.google_jwks[&header.kid],
        &Validation::new(Algorithm::RS256),
    )
    .unwrap()
    .claims;

    return HttpResponse::Ok().body(format!("{:?}", test_token));
}
