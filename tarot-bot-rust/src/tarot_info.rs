use std::{collections::BTreeMap, env, path::Path};

use rand::seq::IteratorRandom;
use serde_json::Value;
use walkdir::WalkDir;

use crate::handler::TarotHandler;

const DATA_PATH: &str = "tarot-bot-rust/data";

pub enum TarotEvent {
    AllTitles,
    Random,
    Single(usize),
}

#[derive(Debug, Default)]
pub struct TarotStore {
    tarot_info: BTreeMap<usize, Value>,
    base_url: String,
}

impl TarotStore {
    pub fn new() -> Self {
        let path = Path::new(DATA_PATH);
        let mut tarot_info = BTreeMap::new();
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }

            if let (Some(file_name), Ok(content)) = (
                path.file_name().and_then(|s| s.to_str()),
                std::fs::read_to_string(path),
            ) {
                if let Some(Ok(index)) = file_name.split('_').next().map(|s| s.parse::<usize>()) {
                    if let Ok(value) = serde_json::from_str::<Value>(&content) {
                        tarot_info.insert(index, value);
                    }
                }
            }
        }
        let base_url = env::var("GITHUB_URL").unwrap_or_default();
        Self {
            tarot_info,
            base_url,
        }
    }

    pub fn get_tarot_info(&self, tarot_info_event: TarotEvent) -> Value {
        match tarot_info_event {
            TarotEvent::AllTitles => self.get_all_tarot_title(),
            TarotEvent::Random => self.get_random_tarot_info(),
            TarotEvent::Single(index) => self.get_single_tarot_info(index, false),
        }
    }

    fn get_all_tarot_title(&self) -> Value {
        let info = self
            .tarot_info
            .iter()
            .map(|(index, info)| {
                let title = info
                    .get("title")
                    .and_then(|v| v.as_str())
                    .unwrap_or("無法獲得卡片");
                format!("{index}: {title}")
            })
            .collect::<Vec<String>>()
            .join("\n");
        TarotHandler::get_msg_body(&info)
    }

    fn get_single_tarot_info(&self, index: usize, is_random_reverse: bool) -> Value {
        let info = self.tarot_info.get(&index).unwrap();
        let title = info
            .get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("未知卡片");
        let desc = info
            .get("short_description")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        let source_url = info
            .get("source_url")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        let msg = format!("{title}\n\n{desc}\n\n{source_url}");

        let url_path = if is_random_reverse && rand::random_bool(0.5) {
            "reversed_path"
        } else {
            "upright_path"
        };

        let image_path = info
            .get(url_path)
            .and_then(|v| v.as_str())
            .unwrap_or_default();

        let image_url = format!("{}/{image_path}", self.base_url);

        TarotHandler::get_image_body(&msg, &image_url)
    }

    fn get_random_tarot_info(&self) -> Value {
        let rand_index = self
            .tarot_info
            .keys()
            .choose(&mut rand::rng())
            .copied()
            .unwrap();
        self.get_single_tarot_info(rand_index, true)
    }
}
