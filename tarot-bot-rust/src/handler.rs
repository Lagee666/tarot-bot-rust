use std::{env, sync::Arc};

use axum::{Json, extract::State};
use serde_json::{Value, json};
use tracing::{debug, error};

use crate::{
    help::get_help,
    tarot_info::{TarotEvent, TarotStore},
};

#[derive(Debug, Default)]
pub struct TarotHandler {
    tarot_store: TarotStore,
    client: reqwest::Client,
    line_token: String,
}

const DRAW: [&str; 8] = [
    "抽卡",
    "抽一張牌",
    "抽一張卡",
    "抽塔羅牌",
    "抽塔羅",
    "抽一張塔羅牌",
    "抽一張塔羅",
    "抽一張",
];
const HELP: [&str; 4] = ["幫助", "說明", "指令", "指令說明"];
const ALL_CARDS: [&str; 1] = ["所有卡片"];

impl TarotHandler {
    pub fn new() -> Self {
        Self {
            tarot_store: TarotStore::new(),
            client: reqwest::Client::new(),
            line_token: env::var("LINE_CHANNEL_ACCESS_TOKEN").expect("LINE_TOKEN not set"),
        }
    }

    pub async fn handle(
        State(handler): State<Arc<TarotHandler>>,
        Json(payload): Json<Value>,
    ) -> &'static str {
        if let Some(event) = payload["events"].get(0) {
            let reply_token = event["replyToken"].as_str();
            let user_message = event["message"]["text"].as_str();
            debug!("Get user msg: {:?}", user_message);
            let body = match user_message {
                Some(msg) if DRAW.contains(&msg) => {
                    handler.tarot_store.get_tarot_info(TarotEvent::Random)
                }
                Some(msg) if ALL_CARDS.contains(&msg) => {
                    handler.tarot_store.get_tarot_info(TarotEvent::AllTitles)
                }
                Some(msg) if HELP.contains(&msg) => get_help(),
                Some(msg) => {
                    if let Ok(index) = msg.parse::<usize>()
                        && (0..=77).contains(&index)
                    {
                        handler
                            .tarot_store
                            .get_tarot_info(TarotEvent::Single(index))
                    } else {
                        get_help()
                    }
                }
                None => Self::get_msg_body("目前僅支援文字指令喔！"),
            };

            if let Some(token) = reply_token {
                handler.send_reply(token, body).await;
            }
        }

        "OK"
    }

    async fn send_reply(&self, reply_token: &str, mut body: Value) {
        if let Some(body) = body.as_object_mut() {
            body.insert("replyToken".to_string(), json!(reply_token));
        }

        let res = self
            .client
            .post("https://api.line.me/v2/bot/message/reply")
            .header("Authorization", format!("Bearer {:?}", self.line_token))
            .json(&body)
            .send()
            .await;

        match res {
            Ok(_) => debug!("Reply sent successfully!"),
            Err(e) => error!("Error sending reply: {:?}", e),
        }
    }

    pub fn get_msg_body(msg: &str) -> Value {
        json!({
            "messages": [
                {
                    "type": "text",
                    "text": msg
                }
            ]
        })
    }

    pub fn get_image_body(msg: &str, url: &str) -> Value {
        json!({
            "messages": [
                {
                    "type": "text",
                    "text": msg
                },
                {
                    "type": "image",
                    "originalContentUrl": url,
                    "previewImageUrl": url
                }
            ]
        })
    }
}
