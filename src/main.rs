use std::net::SocketAddr;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use sqlx::postgres::PgPoolOptions;

use axum::{
    routing::{delete, get, post},
    Router,
};

mod handlers;
mod models;
mod persistance;

use handlers::*;

const HOST: [u8; 4] = [127, 0, 0, 1];
const PORT: u16 = 8000;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file.
    dotenvy::dotenv().expect(
        "dotenv initialization failed. Make sure you have the .env file in the root of the project.");
    pretty_env_logger::init();

    let db_url =
        std::env::var("DATABASE_URL").expect("Environment variable DATABASE_URL is not provided.");

    let _pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("PgPool initialization failed");

    let app = Router::new()
        .route("/question", post(create_question))
        .route("/questions", get(read_questions))
        .route("/question", delete(delete_question))
        .route("/answer", post(create_answer))
        .route("/answers", get(read_answers))
        .route("/answer", delete(delete_answer));

    let addr = SocketAddr::from((HOST, PORT));

    info!(
        "Server is being started, http://{}:{PORT}",
        HOST.map(|v| v.to_string()).join(".")
    );

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
