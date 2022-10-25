use super::IRequest;
use std::collections::HashMap;

/// taobao.tbk.tpwd.convert( 淘宝客-推广者-淘口令解析&转链 )
/// 支持通过淘口令解析商品id，并提供对应的淘客转链接
// https://open.taobao.com/api.htm?docId=43873&docType=2&scopeId=16401
pub struct TbkTpwdConvertRequest {
    pub session:String,//用户登录授权成功后，TOP颁发给应用的授权信息
    pub password_content: String,
    pub adzone_id: String,
    pub dx: String,
    pub site_id: String,
    pub ucrowd_id: String,
    pub relation_id: String,    
}
impl TbkTpwdConvertRequest {
    pub fn new(
        session:&str,
        password_content: &str,
        adzone_id: &str,
        dx: &str,
        site_id: &str,
        ucrowd_id: &str,
        relation_id: &str,
    ) -> Self {
        TbkTpwdConvertRequest {
            session: session.to_string(),
            password_content: password_content.to_string(),
            adzone_id: adzone_id.to_string(),
            dx: dx.to_string(),
            site_id: site_id.to_string(),
            ucrowd_id: ucrowd_id.to_string(),
            relation_id: relation_id.to_string(),
        }
    }
}
impl IRequest for TbkTpwdConvertRequest {
    // 获取方法名
    fn get_method_name(&self) -> String {
        if self.session.len()>0{
            "taobao.tbk.sc.tpwd.convert".to_string()
        }else{
            "taobao.tbk.tpwd.convert".to_string()
        }       
    }
    // 获取请求参数
    fn get_map_param(&self) -> HashMap<&str, String> {
        let mut map: HashMap<&str, String> = HashMap::new();
        map.insert("password_content", self.password_content.clone());
        map.insert("adzone_id", self.adzone_id.clone());

        if self.dx.len() > 0 {
            map.insert("dx", self.dx.clone());
        }
        if self.site_id.len() > 0 {
            map.insert("site_id", self.site_id.clone());
        }
        if self.ucrowd_id.len() > 0 {
            map.insert("ucrowd_id", self.ucrowd_id.clone());
        }
        if self.relation_id.len() > 0 {
            map.insert("relation_id", self.relation_id.clone());
        }
        map
    }
}
