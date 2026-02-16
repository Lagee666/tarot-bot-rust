use serde_json::Value;

use crate::handler::TarotHandler;

pub fn get_help() -> Value {
    let msg = "歡迎使用塔羅牌抽卡機器人！你可以輸入以下指令：
1. 輸入 0-77 的數字，抽取對應編號的塔羅牌。
2. 輸入「抽卡」或類似指令，隨機抽取一張塔羅牌。
3. 輸入「幫助」查看此說明。
4. 輸入「所有卡片」查看所有塔羅牌名稱。
祝你有美好的一天！";
    TarotHandler::get_msg_body(msg)
}
