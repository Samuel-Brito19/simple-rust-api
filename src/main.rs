use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, App, HttpServer};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_webi=info");
    }
    dotenv().ok();
    env_logger::init();

    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: Pool<Postgres> = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Connection successful");
            pool
        }
        Err(err) => {
            println!("Conection refused {:?}", err);
            std::process::exit(1)
        }
    };

    println!("Server starded successfully!");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http:://locahost::3000")
            .allowed_methods(vec!["GET", "POSTS", "DELETE", "PUT"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(actix_web::web::Data::new(AppState { db: pool.clone() }))
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

    //Ok(())
}
