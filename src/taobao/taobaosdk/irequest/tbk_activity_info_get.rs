use std::collections::HashMap;
use super::IRequest;

/// taobao.tbk.activity.info.get( 淘宝客-推广者-官方活动转链 )
// https://open.taobao.com/api.htm?docId=48340&docType=2
pub struct TaobaoTbkActivityInfoGetRequest {
    pub activity_material_id: String,
    pub adzone_id: i64,
    pub sub_pid: String,
    pub relation_id: String,
    pub union_id: String,
}
impl IRequest for TaobaoTbkActivityInfoGetRequest {
    // 获取方法名
    fn get_method_name(&self) -> String {
        "taobao.tbk.activity.info.get".to_string()
    }
    // 获取请求参数
    fn get_map_param(&self) -> HashMap<&str, String> {
        let mut map: HashMap<&str, String> = HashMap::new();
        map.insert("activity_material_id", self.activity_material_id.clone());
        map.insert("adzone_id", self.adzone_id.to_string());
        if self.sub_pid.len() > 0 {
            map.insert("sub_pid", self.sub_pid.clone());
        }
        if self.relation_id.len() > 0 {
            map.insert("relation_id", self.relation_id.clone());
        }
        if self.union_id.len() > 0 {
            map.insert("union_id", self.union_id.clone());
        }
        map
    }
}
