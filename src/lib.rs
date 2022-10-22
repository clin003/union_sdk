//! 开放平台 Rust SDK, 集成简单快捷，也可以快速添加本SDK没有集成的接口。
//! 目前已经实现的功能如下：
//! 
//! ### 淘宝开放平台
//!  * 淘口令生成
//!  * 官方活动转链
//! 


pub mod util;
pub mod taobao;
pub use taobao::taobaosdk;
