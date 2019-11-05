// Copyright © Shaipe
//! 项目对接公用方法

use crate::base64;

/// 获取内容的md5加密码,使用的是128位模式下的base64编码
pub fn content_md5(text: &str) -> String {
    use md5::{Md5, Digest};
    use md5::digest::FixedOutput;
    let mut m = Md5::default();
    m.input(text.as_bytes());
    // 获取原固定的128位的u8
    let digest = m.fixed_result();
    base64::encode(&digest)
}
pub fn content_md52(bytes: &Vec<u8>) -> String {
    use md5::{Md5, Digest};
    use md5::digest::FixedOutput;
    let mut m = Md5::default();
    m.input(bytes);
    // 获取原固定的128位的u8
    let digest = m.fixed_result();
    base64::encode(&digest)
}
/// 获取hmac_sha1的加密码方式
pub fn content_sha1(key:&str, text: &str) -> String {
    use hmacsha1::hmac_sha1;
    // println!("{:?}, text: {:?}", key, text);
    let digest = hmac_sha1(key.as_bytes(),text.as_bytes());
    // println!("{:?}", digest);
    base64::encode(&digest)
}

/// 获取资源名称
pub fn get_resource(bucket_name: &str, object_name: &str, sub_resource: &str,source_path:&str) -> String {
    let resource = if sub_resource != "" {
        format!("?{}", sub_resource)
    }
    else{
        "".to_owned()
    };
    if bucket_name == "" {
        return format!("/{}{}{}", object_name, resource,source_path);
    }

    return format!("/{}{}{}{}", bucket_name, object_name, resource,source_path);
}
/// 获取GMT格式的当前时间
pub fn get_now_gmt() -> String {
    use chrono::{DateTime, Utc};
    let now: DateTime<Utc> = Utc::now();
    let format_time=now.to_rfc2822().to_string().replace("+0000","GMT");
    return format_time;
}

