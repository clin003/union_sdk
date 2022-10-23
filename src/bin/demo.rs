use dotenv::dotenv;
use std::env;
use union_sdk::taobaosdk::{SDKClient, TaobaoTbkTpwdCreateRequest};
// cargo run --package union_sdk --bin demo
#[tokio::main]
async fn main() {
    // 这里使用 ok() 的目的就是当加载 dotenv 环境文件失败的时候可以忽略错误。
    dotenv().ok();

    let app_key = env::var("APP_KEY").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();

    let tkl_url = env::var("TKL_URL").unwrap();
    // https://open.taobao.com/api.htm?docId=31127&docType=2
    let arg = TaobaoTbkTpwdCreateRequest::new(tkl_url.as_str());
    let result_instance = SDKClient::new(app_key.as_str(), app_secret.as_str(), "")
        .send_request(arg)
        .await
        .unwrap();
    println!("{}", result_instance);
}
