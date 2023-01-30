use actix_web::{
    get,
    web::{self, scope},
    App, HttpResponse, HttpServer, Responder,
};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct AppState {
    pub words_list: Vec<String>,
    pub quotes_list: Vec<QouteEntry>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QouteEntry {
    pub quote: String,
    pub author: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

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
                std::fs::read_to_string("./quotes.json")
                    .unwrap()
                    .as_str(),
            )
            .unwrap(),
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
                    .service(get_quotes_entry),
            )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
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
