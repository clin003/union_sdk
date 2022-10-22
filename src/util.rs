//! util 常用方法
use hex;
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;

/// hmac_sha256 加密算法
///
/// # Examples
///
/// ```
/// use union_sdk::util;
/// let (message,key)=("test","key");
/// let ret=util::hmac_sha256_hex(message.as_bytes(),key.as_bytes());
/// println!("{:?}",ret);
/// ```
///
pub fn hmac_sha256_hex(message: &[u8], key: &[u8]) -> String {
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(message);
    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    let code_slice = code_bytes.as_slice();
    hex::encode(code_slice)
}

// pub fn hmac_sha256(message: &[u8], key: &[u8]) -> [u8; 32] {
//     type HmacSha256 = Hmac<Sha256>;
//     let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");
//     mac.update(message);
//     let result = mac.finalize();

//     let code_bytes = result.into_bytes();
//     let code_slice = code_bytes.as_slice();
//     code_slice.try_into().expect("slice with incorrect length")
// }

// use std::collections::HashMap;
// /// taobao TOPSDK sign
// pub fn get_taobao_topsdk_sign(
//     pub_param: HashMap<&str, String>,
//     req_param: HashMap<&str, String>,
//     app_secret: String,
// ) -> String {
//     // let mut ret_sign: String = String::from("");
//     let mut key_list: Vec<&str> = Vec::new();
//     let mut all_param_map: HashMap<&str, String> = HashMap::new();

//     for (key, val) in pub_param {
//         all_param_map.insert(key, val);
//         key_list.push(key)
//     }
//     for (key, val) in req_param {
//         all_param_map.insert(key, val);
//         key_list.push(key)
//     }
//     key_list.sort();
//     let mut sign_str = String::new();
//     for key in key_list {
//         let value = all_param_map[key].clone();
//         sign_str = format!("{}{}{}", sign_str, key, value)
//     }

//     hmac_sha256_hex(sign_str.as_str().as_bytes(), app_secret.as_str().as_bytes())
//         .to_ascii_uppercase()
// }
