use crate::{endpoints::auth::User, structs::RankedResponse, utils::verify_token, AppState};

use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use base64::{engine::general_purpose, Engine as _};
use serde::Serialize;
use std::io::Write;

#[derive(sqlx::FromRow, Serialize)]
pub struct RankedQuote {
    pub id: i32,
    pub start_at: i64,
    pub quote: String,
}

#[get("")]
pub async fn get_ranked_quote(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let user: User = match verify_token(&req, &data.pool).await {
        Some(user) => user,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let ranked_quote: RankedQuote = sqlx::query_as(
        "SELECT * FROM ranked_quotes WHERE start_at < extract(epoch from now()) 
         ORDER BY start_at DESC LIMIT 1",
    )
    .fetch_one(&data.pool)
    .await
    .unwrap();

    return match sqlx::query("INSERT INTO r_results(user_id, quote_id) VALUES ($1, $2)")
        .bind(user.id)
        .bind(ranked_quote.id)
        .execute(&data.pool)
        .await
    {
        Ok(_) => HttpResponse::Ok().json(ranked_quote),
        Err(_) => HttpResponse::Forbidden().body("You've already started that quote!"),
    };
}

#[post("")]
pub async fn ranked_response(
    data: web::Data<AppState>,
    response_data: web::Json<RankedResponse>,
    req: HttpRequest,
) -> impl Responder {
    let user: User = match verify_token(&req, &data.pool).await {
        Some(user) => user,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let wpm_time = response_data.time as f64 / 60000f64;
    let wpm = response_data.chars_in_correct_words as f64 / 5f64 / wpm_time;

    let accuracy = response_data.chars_correct as f64 / response_data.chars_written as f64 * 100f64;

    let input = response_data.history.as_bytes();
    let mut writer = brotli::CompressorWriter::new(Vec::new(), 4096, 11, 22);
    writer.write_all(&input).unwrap();

    let compressed_history = general_purpose::STANDARD.encode(writer.into_inner());

    let insert_query = sqlx::query(
        "UPDATE r_results SET time = $1, wpm = $2, acc = $3, ks_history = $4, 
         submitted_at = extract(epoch from now())
         WHERE submitted_at IS NULL AND user_id = $5 AND quote_id = $6",
    )
    .bind(response_data.time)
    .bind(wpm)
    .bind(accuracy)
    .bind(compressed_history)
    .bind(user.id)
    .bind(response_data.quote_id)
    .execute(&data.pool)
    .await;

    return match insert_query {
        Ok(affected) => {
            if affected.rows_affected() == 0 {
                return HttpResponse::InternalServerError().finish();
            }
            return HttpResponse::Ok().finish();
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    };
}
