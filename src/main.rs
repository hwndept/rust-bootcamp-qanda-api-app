use std::net::SocketAddr;

use axum::{
    routing::{delete, get, post},
    Router,
};

mod handlers;
mod models;

use handlers::*;

const HOST: [u8; 4] = [127, 0, 0, 1];
const PORT: u16 = 8000;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/question", post(create_question))
        .route("/questions", get(read_questions))
        .route("/question", delete(delete_question))
        .route("/answer", post(create_answer))
        .route("/answers", get(read_answers))
        .route("/answer", delete(delete_answer));

    let addr = SocketAddr::from((HOST, PORT));

    println!(
        "Server is being started, http://{}:{PORT}",
        HOST.map(|v| v.to_string()).join(".")
    );

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
