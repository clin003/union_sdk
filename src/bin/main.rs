use dotenv::dotenv;
use std::env;
use union_sdk::taobaosdk::{SDKClient, TaobaoTbkActivityInfoGetRequest};
// cargo run --package union_sdk --bin main
#[tokio::main]
async fn main() {
    // 这里使用 ok() 的目的就是当加载 dotenv 环境文件失败的时候可以忽略错误。
    dotenv().ok();

    let app_key = env::var("APP_KEY").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    let activity_material_id=env::var("MATERIAL_ID").unwrap();

    // https://open.taobao.com/api.htm?docId=48340&docType=2&scopeId=18294
    let arg = TaobaoTbkActivityInfoGetRequest {
        activity_material_id: activity_material_id.to_string(),
        adzone_id: 111121450199 as i64,
        sub_pid: "".to_owned(),
        relation_id: "".to_owned(),
        union_id: "".to_owned(),
    };

    let result_instance = SDKClient::new(app_key.as_str(), app_secret.as_str(), "")
        .send_request(arg)
        .await
        .unwrap();
    println!("{}", result_instance);
}
