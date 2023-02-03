use actix_web::{get, post, web, HttpResponse, Responder};
use rand::Rng;

use crate::{
    structs::{HistoryEntry, KeyracerData, KeyracerResponse},
    AppState,
};

#[get("")]
pub async fn get_index(data: web::Data<AppState>) -> impl Responder {
    let row: (String,) = sqlx::query_as("SELECT 'Hello, world!'")
        .fetch_one(&data.pool)
        .await
        .unwrap();

    return HttpResponse::Ok().body(format!("{:?}", row));
}

#[get("/words/{count}")]
pub async fn get_test(data: web::Data<AppState>, path: web::Path<i64>) -> impl Responder {
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
pub async fn get_quotes_entry(data: web::Data<AppState>) -> impl Responder {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..data.quotes_list.len());

    return HttpResponse::Ok().json(data.quotes_list[index].clone());
}

#[get("/test")]
pub async fn test() -> impl Responder {
    return HttpResponse::Ok()
        .body(std::fs::read_to_string("/home/notpilif/Downloads/test.html").unwrap());
}
