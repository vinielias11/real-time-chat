use std::sync::Arc;

use ::axum::{Router, http::Method, http::header, routing::get, routing::post};
use ::dotenv::dotenv;
use ::tower_http::cors::{AllowOrigin, CorsLayer};
use sqlx::{PgPool, postgres::PgPoolOptions};

mod handler;
mod model;
mod schema;

pub struct AppState {
    db: PgPool,
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
        .route("/api/usuarios", post(handler::usuario_handler::create))
        .route( "/api/chat", get(handler::chat_handler::websocket_handler))
        .with_state(Arc::new(AppState { db: pool.clone() }))
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::any())
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
                .expose_headers(["X-Custom-Header".parse().unwrap()]),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!("Servidor iniciado na porta 8080!");

    axum::serve(listener, app).await.unwrap();
}
