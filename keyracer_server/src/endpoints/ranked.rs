use crate::{endpoints::auth::User, structs::RankedResponse, utils::verify_token, AppState};

use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(sqlx::FromRow, Serialize)]
pub struct RankedQuote {
    pub id: i32,
    pub start_at: i64,
    pub quote: String,
}

#[derive(sqlx::FromRow, Serialize)]
pub struct RankingHistoryEntry {
    pub id: i32,
    pub quote: String,
    pub start_at: i64,
}

#[derive(sqlx::FromRow, Serialize)]
pub struct RankingEntry {
    pub id: i32,
    pub name: String,
    pub time: i32,
    pub wpm: f64,
    pub acc: f64,
    pub submitted_at: i64,
}

#[get("")]
pub async fn get_ranked_avail(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let user: User = match verify_token(&req, &data.pool).await {
        Some(user) => user,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let latest_ranked =
        sqlx::query("
                    SELECT id FROM r_results WHERE user_id = $1 AND 
                    quote_id = (SELECT id FROM ranked_quotes WHERE start_at < extract(epoch from now()) 
                    ORDER BY start_at DESC LIMIT 1)")
            .bind(user.id)
            .fetch_one(&data.pool)
            .await;

    return match latest_ranked {
        Ok(_) => HttpResponse::Forbidden().body("No events for you!"),
        Err(_) => HttpResponse::Ok().finish(),
    };
}

#[post("/start")]
pub async fn start_ranked_quote(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
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

#[post("/submit")]
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

#[derive(Deserialize, Debug)]
pub struct RankingQuery {
    pub limit: Option<i32>,
}

#[get("/ranking")]
pub async fn get_ranked_history(
    data: web::Data<AppState>,
    query: web::Query<RankingQuery>,
) -> impl Responder {
    let limit: i32 = match query.limit {
        Some(limit) => limit,
        None => 10,
    };

    let entries: Result<Vec<RankingHistoryEntry>, sqlx::Error> = sqlx::query_as(
        "SELECT id, quote, start_at FROM ranked_quotes 
         WHERE start_at < extract(epoch from now()) 
         ORDER BY start_at DESC LIMIT $1",
    )
    .bind(limit)
    .fetch_all(&data.pool)
    .await;

    return match entries {
        Ok(entries) => HttpResponse::Ok().json(entries),
        Err(_) => HttpResponse::InternalServerError().finish(),
    };
}

#[get("/ranking/{quote_id}")]
pub async fn get_ranked_ranking(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let quote_id = path.into_inner();

    let entries: Result<Vec<RankingEntry>, sqlx::Error> = sqlx::query_as(
        "SELECT r_results.id, users.name, r_results.time, r_results.wpm, r_results.acc, r_results.submitted_at  
         FROM r_results INNER JOIN users ON r_results.user_id = users.id 
         WHERE quote_id = $1 ORDER BY wpm DESC",
    )
    .bind(quote_id)
    .fetch_all(&data.pool)
    .await;

    return match entries {
        Ok(entries) => HttpResponse::Ok().json(entries),
        Err(_) => HttpResponse::InternalServerError().finish(),
        //Err(err) => HttpResponse::InternalServerError().body(format!("{:?}", err)),
    };
}
