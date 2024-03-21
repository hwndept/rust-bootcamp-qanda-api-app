#[macro_use]
extern crate log;

extern crate pretty_env_logger;

use std::{net::SocketAddr, sync::Arc};

use axum::{
    routing::{delete, get, post},
    Router,
};

use persistance::{
    answers_dao::{AnswersDao, AnswersDaoImpl},
    questions_dao::{QuestionsDao, QuestionsDaoImpl},
};
use sqlx::postgres::PgPoolOptions;

mod handlers;
mod models;
mod persistance;

use handlers::*;

const HOST: [u8; 4] = [127, 0, 0, 1];
const PORT: u16 = 8000;

#[derive(Clone)]
pub struct AppState {
    pub questions_dao: Arc<dyn QuestionsDao + Send + Sync>,
    pub answers_dao: Arc<dyn AnswersDao + Send + Sync>,
}

#[tokio::main]
async fn main() {
    // Load environment variables from .env file.
    dotenvy::dotenv().expect(
        "dotenv initialization failed. Make sure you have the .env file in the root of the project.");

    pretty_env_logger::init();

    let db_url =
        std::env::var("DATABASE_URL").expect("Environment variable DATABASE_URL is not provided.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to create Postgres connection pool!");

    let questions_dao = Arc::new(QuestionsDaoImpl::new(pool.clone()));
    let answers_dao = Arc::new(AnswersDaoImpl::new(pool));

    let app_state = AppState {
        questions_dao,
        answers_dao,
    };

    let app = Router::new()
        .route("/questions", post(create_question))
        .route("/questions", get(read_questions))
        .route("/questions", delete(delete_question))
        .route("/answers", post(create_answer))
        .route("/answers", get(read_answers))
        .route("/answers", delete(delete_answer))
        .with_state(app_state);

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
