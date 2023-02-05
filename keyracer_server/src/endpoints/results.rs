use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};

use crate::{
    endpoints::auth::User,
    structs::{HistoryEntry, KeyracerData, KeyracerResponse, NrResult},
    utils::verify_token,
    AppState,
};

#[post("")]
pub async fn keyracer_response(
    data: web::Data<AppState>,
    response_data: web::Json<KeyracerResponse>,
    req: HttpRequest,
) -> impl Responder {
    let user: User = match verify_token(&req, &data.pool).await {
        Some(user) => user,
        None => return HttpResponse::Unauthorized().finish(),
    };

    // this _data is useless for now, but it will be used in the future when I will add ranking
    // system
    let _data: KeyracerData = KeyracerData {
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

    let mut max_ks_time = -1;
    for i in 1.._data.history.len() {
        let ks_time = _data.history[i].time - _data.history[i - 1].time;

        if ks_time > max_ks_time {
            max_ks_time = ks_time;
        }
    }

    let wpm_time = response_data.time as f64 / 60000f64;
    let wpm = response_data.chars_in_correct_words as f64 / 5f64 / wpm_time;

    let time = response_data.time;

    let accuracy = response_data.chars_correct as f64 / response_data.chars_written as f64 * 100f64;

    let insert_query = sqlx::query(
        "INSERT INTO nr_results(user_id, time, wpm, acc, max_ks) VALUES ($1, $2, $3, $4, $5) RETURNING id",
    )
    .bind(user.id)
    .bind(time)
    .bind(wpm)
    .bind(accuracy)
    .bind(max_ks_time)
    .fetch_one(&data.pool)
    .await;

    return match insert_query {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    };
}

#[get("/{user_id}")]
pub async fn get_user_results(data: web::Data<AppState>, path: web::Path<i64>) -> impl Responder {
    let results: Result<Vec<NrResult>, sqlx::Error> = sqlx::query_as(
        "SELECT * FROM nr_results WHERE user_id = $1 ORDER BY created_at DESC LIMIT 50",
    )
    .bind(path.into_inner())
    .fetch_all(&data.pool)
    .await;

    return match results {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(err) => HttpResponse::InternalServerError().body(format!("{:?}", err)),
    };
}
