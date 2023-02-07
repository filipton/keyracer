use actix_web::{get, web, HttpResponse, Responder};

use crate::AppState;

#[get("/words")]
pub async fn get_words(data: web::Data<AppState>) -> impl Responder {
    return HttpResponse::Ok().body(data.words_list.clone());
}

#[get("/quotes")]
pub async fn get_quotes(data: web::Data<AppState>) -> impl Responder {
    return HttpResponse::Ok()
        .insert_header(("Content-Type", "application/json"))
        .body(data.quotes_list.clone());
}
