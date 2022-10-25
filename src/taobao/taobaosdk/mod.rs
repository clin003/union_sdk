//! 淘宝官方开放平台SDK
//! 
//! # Examples
//! 
//! ```
//! use union_sdk::taobaosdk::{SDKClient, TaobaoTbkTpwdCreateRequest};
//!
//!     let app_key = "";
//!     let app_secret = "";
//!     let tkl_url="";
//!     let arg=TaobaoTbkTpwdCreateRequest::new(tkl_url.as_str());
//!     let result_instance = SDKClient::new(app_key, app_secret, "")
//!         .send_request(arg)
//!         .await
//!         .unwrap();
//!     println!("{}", result_instance);
//! 
//! ```


mod sdkclient;
pub use self::sdkclient::SDKClient;

mod irequest;
pub use self::irequest::IRequest;
/// 官方活动转链
pub use self::irequest::tbk_activity_info_get::TaobaoTbkActivityInfoGetRequest;
/// 淘口令生成
pub use self::irequest::tbk_tpwd_create::TaobaoTbkTpwdCreateRequest;
/// 淘口令解析&转链
pub use self::irequest::tbk_tpwd_convert::TbkTpwdConvertRequest;
/// 淘口令解析&&转链（临时接口）
pub use self::irequest::tbk_tpwd_temporary_convert::TbkTpwdTemporaryConvertRequest;

