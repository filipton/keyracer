use actix_web::{get, web::scope, App, HttpResponse, HttpServer, Responder};

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
        let cors = actix_cors::Cors::default()
            .allow_any_origin()
            .send_wildcard()
            .allow_any_method()
            .allow_any_header()
            .max_age(None);

        App::new()
            .wrap(cors)
            // .app_data(web::Data::new({}))
            .service(scope("").service(get_index).service(get_test))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

#[get("/")]
async fn get_index() -> impl Responder {
    return HttpResponse::Ok().body("Nothing here yet!");
}

#[get("/test")]
async fn get_test() -> impl Responder {
    return HttpResponse::Ok().body("Nothing here yet! 2");
}
