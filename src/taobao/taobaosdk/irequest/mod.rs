// use serde;
// use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub trait IRequest {
    fn get_method_name(&self) -> String;
    fn get_map_param(&self) -> HashMap<&str, String>;
}

pub mod tbk_activity_info_get;
pub mod tbk_tpwd_create;

// use std::collections::HashMap;

// // #[derive(Serialize, Deserialize)]
// // #[serde(rename_all = "PascalCase")]
// pub struct TaobaoTbkActivityInfoGetRequest {
//     pub activity_material_id: String,
//     pub adzone_id: i64,
//     pub sub_pid: String,
//     pub relation_id: String,
//     pub union_id: String,
// }
// impl IRequest for TaobaoTbkActivityInfoGetRequest {
//     // 获取方法名
//     fn get_method_name(&self) -> String {
//         "taobao.tbk.activity.info.get".to_string()
//     }
//     // 获取请求参数
//     fn get_map_param(&self) -> HashMap<&str, String> {
//         let mut map: HashMap<&str, String> = HashMap::new();
//         map.insert("activity_material_id", self.activity_material_id.clone());
//         map.insert("adzone_id", self.adzone_id.to_string());
//         if self.sub_pid.len() > 0 {
//             map.insert("sub_pid", self.sub_pid.clone());
//         }
//         if self.relation_id.len() > 0 {
//             map.insert("relation_id", self.relation_id.clone());
//         }
//         if self.union_id.len() > 0 {
//             map.insert("union_id", self.union_id.clone());
//         }
//         map
//     }
//     // // 拼接请求PostBody
//     // pub fn get_req_body(&self) -> String {
//     //     let mut req_body:String = "".to_string();
//     //     for (key, val) in self.get_map_param() {
//     //         let q = format!("{}={}", key, val);
//     //         if req_body.len() > 0 {
//     //             req_body = format!("{}&{}", req_body, q.clone());
//     //         } else {
//     //             req_body = q.clone();
//     //         }
//     //     }
//     //     req_body
//     // }
// }

// // #[derive(Serialize, Deserialize)]
// // #[serde(rename_all = "PascalCase")]
// // pub struct ResResponseCreate {
// //     pub data: ResResponseCreateContent,
// //     pub error_response: Option<ResResponseError>,
// //     pub request_id: String,
// // }

// // #[derive(Serialize, Deserialize)]
// // #[serde(rename_all = "PascalCase")]
// // pub struct ResResponseCreateContent {
// //     pub click_url: String,
// //     pub short_click_url: Option<String>,
// //     pub terminal_type: Option<String>,
// //     pub material_oss_url: Option<String>,
// //     pub page_name: Option<String>,
// //     pub page_start_time: Option<String>,
// //     pub page_end_time: Option<String>,
// //     pub wx_miniprogram_path: Option<String>,
// // }

// // #[derive(Serialize, Deserialize)]
// // #[serde(rename_all = "PascalCase")]
// // pub struct ResResponseError {
// //     pub code: Option<i64>,
// //     pub msg: Option<String>,
// //     pub sub_code: Option<String>,
// //     pub sub_msg: Option<String>,
// //     pub request_id: Option<String>,
// // }

// // #[cfg(test)]
// // mod tests {
// //     use super::*;

// //     #[test]
// //     fn test_json_model() {
// //         let raw = r#"{
// //             "error_response":{
// //                 "msg":"Remote service error",
// //                 "code":50,
// //                 "sub_msg":"非法参数",
// //                 "sub_code":"isv.invalid-parameter"
// //             }
// //         }"#;
// //         let _mode: ResResponseError = serde_json::from_str(raw).expect("msg");
// //     }
// // }
