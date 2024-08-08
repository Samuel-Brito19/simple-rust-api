use actix_web::{web, HttpResponse, Responder};

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

#[post("/game/game/{id}")]
async fn create_note_handler(
    body: web::Json<CreateNoteSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query_as!(
        NoteModel,
        "INSERT INTO notes (title,content,category) VALUES ($1, $2, $3) RETURNING *",
        body.title.to_string(),
        body.content.to_string(),
        body.category.to_owned().unwrap_or("".to_string())
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(note) => {
            let note_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "note": note
            })});

            return HttpResponse::Ok().json(note_response);
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

#[patch("/notes/{id}")]
async fn edit_note_handler(
    path: web::Path<uuid::Uuid>,
    body: web::Json<UpdateNoteSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let note_id = path.into_inner();
    let query_result = sqlx::query_as!(NoteModel, "SELECT * FROM notes WHERE id = $1", note_id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let message = format!("Note with ID: {} not found", note_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail","message": message}));
    }

    let now = Utc::now();
    let note = query_result.unwrap();

    let query_result = sqlx::query_as!(
        NoteModel,
        "UPDATE notes SET title = $1, content = $2, category = $3, published = $4, updated_at = $5 WHERE id = $6 RETURNING *",
        body.title.to_owned().unwrap_or(note.title),
        body.content.to_owned().unwrap_or(note.content),
        body.category.to_owned().unwrap_or(note.category.unwrap()),
        body.published.unwrap_or(note.published.unwrap()),
        now,
        note_id
    )
    .fetch_one(&data.db)
    .await
    ;

    match query_result {
        Ok(note) => {
            let note_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "note": note
            })});

            return HttpResponse::Ok().json(note_response);
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": message}));
        }
    }
}
