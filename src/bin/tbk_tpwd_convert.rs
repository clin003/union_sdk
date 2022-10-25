use dotenv::dotenv;
use std::env;
use union_sdk::taobaosdk::*;
// cargo run --package union_sdk --bin main
#[tokio::main]
async fn main() {
    // 这里使用 ok() 的目的就是当加载 dotenv 环境文件失败的时候可以忽略错误。
    dotenv().ok();

    let app_key = env::var("APP_KEY").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    let adzone_id = env::var("ADZONE_ID").unwrap();
    let tkl=env::var("TKL").unwrap();

    // https://open.taobao.com/api.htm?docId=63323&docType=2&scopeId=16290
    // https://open.taobao.com/api.htm?docId=32932&docType=2&scopeId=16290

    let arg = TbkTpwdConvertRequest::new(
        "",
        tkl.as_str(),
        adzone_id.as_str(),
        "",
        "",
        "",
        "",
    );

    let result_instance = SDKClient::new(app_key.as_str(), app_secret.as_str(), "")
        .send_request(arg)
        .await
        .unwrap();
    println!("{}", result_instance);
}
