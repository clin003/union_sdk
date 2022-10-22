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
pub use self::irequest::tbk_activity_info_get::TaobaoTbkActivityInfoGetRequest;
pub use self::irequest::tbk_tpwd_create::TaobaoTbkTpwdCreateRequest;
