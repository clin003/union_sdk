use std::collections::HashMap;
pub trait IRequest {
    fn get_method_name(&self) -> String;
    fn get_map_param(&self) -> HashMap<&str, String>;
}

pub mod tbk_activity_info_get;
pub mod tbk_tpwd_create;
