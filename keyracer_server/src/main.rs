use ::keyracer_server::endpoints;
use actix_web::{web, App, HttpServer};
use keyracer_server::utils::get_google_certs;
use keyracer_server::AppState;

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

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(100)
        .connect(&std::env::var("DB_URL").unwrap())
        .await
        .unwrap();

    HttpServer::new(move || {
        {
            let app_state: AppState = AppState {
                pool: pool.clone(),

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
        }
        .service(
            web::scope("/api/auth")
                .service(endpoints::auth::get_user_info)
                .service(endpoints::auth::google_auth)
                .service(endpoints::auth::auth_session),
        )
        .service(
            web::scope("/api/results")
                .service(endpoints::results::keyracer_response)
                .service(endpoints::results::get_user_results),
        )
        .service(
            web::scope("/api/ranked")
                .service(endpoints::ranked::get_ranked_avail)
                .service(endpoints::ranked::start_ranked_quote)
                .service(endpoints::ranked::ranked_response),
        )
        .service(
            web::scope("/api/test")
                .service(endpoints::test::get_index)
                .service(endpoints::test::get_test)
                .service(endpoints::test::test)
                .service(endpoints::test::get_quotes_entry),
        )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
