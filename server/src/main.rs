use std::sync::Arc;

mod handler;
mod model;
mod schema;

use::axum::{ Router, routing::get, routing::post };
use::dotenv::dotenv;

use handler::usuario_handler::hello_world;
use sqlx::{PgPool, postgres::PgPoolOptions};
pub struct AppState {
    db: PgPool
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await

    {
        Ok(pool) => {
            println!("Conectou no PostgreSQL!");
            pool
        }

        Err(err) => {
            println!("Falha ao conectar no PostgreSQL: {}", err);
            std::process::exit(1)
        }
    };

    let app = Router::new()
        .route("/api", get(hello_world))
        .route("/api/usuarios", post(handler::usuario_handler::create))
        .with_state(Arc::new(AppState { db: pool.clone() }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Servidor iniciado na porta 3000!");

    axum::serve(listener, app).await.unwrap();
}