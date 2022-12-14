use chrono;
use reqwest::Client;
use reqwest::Result;
use std::collections::HashMap;
// use crate::util;

const TAOBAO_ROUTER: &str = "https://eco.taobao.com/router/rest";

pub struct SDKClient {
    method: String, //todo
    app_key: String,
    app_secret: String,
    session: String,
    timestamp: String, //todo
    v: String,
    sign_method: String,
    sign: String, //todo
    format: String,
    simplify: bool,
}
impl SDKClient {
    // 新建sdk client
    pub fn new(app_key: &str, app_secret: &str, session: &str) -> Self {
        let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        SDKClient {
            method: "".to_string(),
            app_key: app_key.to_string(),
            app_secret: app_secret.to_string(),
            session: session.to_string(),
            timestamp: timestamp,
            v: "2.0".to_string(),
            sign_method: "hmac-sha256".to_string(),
            sign: "".to_string(),
            format: "json".to_string(),
            simplify: true,
        }
    }
    // 发送请求
    pub async fn send_request(&mut self, req: impl super::IRequest) -> Result<String> {
        if self.app_key.len() == 0 || self.app_secret.len() == 0 {
            return Ok("".to_string());
        }
        self.method = req.get_method_name();
        let req_param = req.get_map_param();
        self.make_sign(req_param.clone());

        let response = Client::new()
            .request(
                reqwest::Method::from_bytes("POST".as_bytes()).unwrap(),
                reqwest::Url::parse(TAOBAO_ROUTER).expect("not a valid url"),
            )
            .header(
                "Content-Type",
                "application/x-www-form-urlencoded;charset=utf-8",
            )
            // .header("Host", self.host.as_str())
            // .body(req.get_req_body())
            .query(&self.get_map_param())
            .form(&req_param.clone())
            .send()
            .await?;
        // response.json().await
        response.text().await
    }

    // 获取公共参数
    fn get_map_param(&self) -> HashMap<&str, String> {
        // let mut map: HashMap<String, String> = HashMap::new();
        let mut map = HashMap::from([
            ("method", self.method.clone()),
            ("app_key", self.app_key.clone()),
            ("v", self.v.clone()),
            ("sign_method", self.sign_method.clone()),
            ("format", self.format.clone()),
            ("simplify", self.simplify.to_string()),
            ("timestamp", self.timestamp.clone()),
            // ("sign",self.sign),
        ]);
        if self.sign.len() > 0 {
            map.insert("sign", self.sign.clone());
        }
        if self.session.len() > 0 {
            map.insert("session", self.session.clone());
        }
        map
    }

    // 生成签名
    fn make_sign(&mut self, req_param: HashMap<&str, String>) {
        let pub_param = self.get_map_param();
        let mut key_list: Vec<&str> = Vec::new();
        let mut all_param_map: HashMap<&str, String> = HashMap::new();

        for (key, val) in pub_param {
            all_param_map.insert(key, val);
            key_list.push(key)
        }
        for (key, val) in req_param {
            all_param_map.insert(key, val);
            key_list.push(key)
        }
        key_list.sort();
        let mut sign_str = String::new();
        for key in key_list {
            let value = all_param_map[key].clone();
            sign_str = format!("{}{}{}", sign_str, key, value)
        }

        self.sign = crate::util::hmac_sha256_hex(
            sign_str.as_str().as_bytes(),
            self.app_secret.as_str().as_bytes(),
        )
        .to_ascii_uppercase();
    }
}
