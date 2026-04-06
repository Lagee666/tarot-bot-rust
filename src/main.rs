use std::sync::Arc;

use axum::{Router, routing::post};
use tarot_bot_rust::handler::TarotHandler;

#[tokio::main]
async fn main() {
    let tarot_handler = Arc::new(TarotHandler::new());
    let tarot_bot_router = Router::new()
        .route("/webhook", post(TarotHandler::handle))
        .with_state(tarot_handler);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000")
        .await
        .unwrap();
    println!("🚀 伺服器啟動：http://localhost:5000");
    axum::serve(listener, tarot_bot_router).await.unwrap();
}
