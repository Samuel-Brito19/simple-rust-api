use actix_web::web;

use crate::AppState;

#[get("/games")]
pub async fn get_games(data: web::Data<AppState>) -> impl Responder {
    let query_result = sqlx::query_as!(GameModel, "SELECT * FROM games")
        .fetch_all(&data.db)
        .await;

    if query_result.is_err() {
        let message: &str = "Failed to fetch games";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error", "message": message}));
    }

    let games = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "no. games": games.len(),
        "games": games
    });

    HttpResponse::Ok().json(json_response)
}

#[post("/games/game")]
async fn create_game(
    body: web::Json<CreateGameSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query_as!(
        "INSERT INTO games (name, description, price) VALUES ($1, $2, $3) RETURNING *",
        body.field_name.to_string(),
        body.address.to_string(),
        body.day.to_string()
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(game) => {
            let game_response = serde_json::json!({
                "status": "success",
                "data": serde_json::json!({
                    "game": game
                })
            });
            return HttpResponse::Ok().json(game_response);
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                return HttpResponse::BadRequest()
                .json(serde_json::json!({"status": "fail","message": "Note with that title already exists"}));
            }

            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    }
}

#[get("/games/game/{id}")]
async fn get_game_by_id(path: web::Path<uuid::Uuid>, data: web::Data<AppState>) -> impl Responder {
    let game_id = path.into_inner();
    let query_result = sqlx::query_as!(GameModel, "SELECT * FROM games WHERE id = $1", game_id)
        .fetch_one(&data.db)
        .await;
    match query_result {
        Ok(game) => {
            let game_response = serde_json::json!({
                "status": "success",
                "data": serde_json::json!({
                    "game": game
                })
            });
            return HttpResponse::Ok().json(game_response);
        }
        Err(_) => {
            let message = format!("Note with id {} not found", game_id);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail","message": message}));
        }
    }
}

#[put("/games/game/{id}")]
async fn update_game_by(
    path: web::Path<uuid::Uuid, data: web::Data<AppState>, body: web::Json<UpdateGamesSchema>>,
) -> impl Responder {
    let game_id = path.into_inner();
}
