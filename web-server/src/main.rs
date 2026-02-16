use std::sync::Arc;

use axum::{Router, routing::post};
use tarot_bot_rust::handler::TarotHandler;
use tracing::{Level, level_filters::LevelFilter};
use tracing_subscriber::{fmt, layer::SubscriberExt};

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::registry()
        .with(LevelFilter::from_level(Level::DEBUG))
        .with(fmt::layer());

    tracing::subscriber::set_global_default(subscriber).expect("failed to set subscriber");

    dotenv::dotenv().ok();
    let tarot_handler = Arc::new(TarotHandler::new());

    let app = Router::new()
        .route("/webhook", post(TarotHandler::handle))
        .with_state(tarot_handler);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
