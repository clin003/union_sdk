use std::collections::HashMap;
use super::IRequest;

/// taobao.tbk.tpwd.create( 淘宝客-公用-淘口令生成 )
/// 提供淘口令生成接口。
// https://open.taobao.com/api.htm?docId=31127&docType=2
pub struct TaobaoTbkTpwdCreateRequest {
    pub url: String,
    pub text: String,
    pub logo: String,
    pub ext: String,
    pub user_id: String,
}
impl TaobaoTbkTpwdCreateRequest {
    pub fn new(url: &str) -> Self {      
        TaobaoTbkTpwdCreateRequest {
            url: url.to_string(),
            text: "".to_string(),
            logo: "".to_string(),
            ext: "".to_string(),
            user_id: "".to_string(),
        }
    }
}
impl IRequest for TaobaoTbkTpwdCreateRequest {
    // 获取方法名
    fn get_method_name(&self) -> String {
        "taobao.tbk.tpwd.create".to_string()
    }
    // 获取请求参数
    fn get_map_param(&self) -> HashMap<&str, String> {
        let mut map: HashMap<&str, String> = HashMap::new();
        map.insert("url", self.url.clone());

        if self.text.len() > 0 {
            map.insert("text", self.text.clone());
        }
        if self.logo.len() > 0 {
            map.insert("logo", self.logo.clone());
        }
        if self.ext.len() > 0 {
            map.insert("ext", self.ext.clone());
        }
        if self.user_id.len() > 0 {
            map.insert("user_id", self.user_id.clone());
        }
        map
    }
}
