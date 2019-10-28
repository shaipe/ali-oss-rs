
use crate::consts::OSSHeaders;
use crate::config::OSSConfig;

#[derive(Clone, Debug)]
pub struct Auth {
    method: String,
    content_md5: Option<String>,
    content_type: String,
    config: OSSConfig
}

use reqwest::header::{HeaderMap};

impl Auth{

    pub fn new(cnf: OSSConfig) -> Self {
        Auth {
            config: cnf,
            method: "POST".to_owned(),
            content_md5: None,
            content_type: String::new()
        }
    }

    /**
 * 
Authorization = "OSS " + AccessKeyId + ":" + Signature
Signature = base64(hmac-sha1(AccessKeySecret,
            VERB + "\n"
            + Content-MD5 + "\n" 
            + Content-Type + "\n" 
            + Date + "\n" 
            + CanonicalizedOSSHeaders
            + CanonicalizedResource))
AccessKeySecret 表示签名所需的密钥。
VERB表示HTTP 请求的Method，主要有PUT、GET、POST、HEAD、DELETE等。
\n 表示换行符。
Content-MD5 表示请求内容数据的MD5值，对消息内容（不包括头部）计算MD5值获得128比特位数字，对该数字进行base64编码而得到。该请求头可用于消息合法性的检查（消息内容是否与发送时一致），如”eB5eJF1ptWaXm4bijSPyxw==”，也可以为空。详情请参见RFC2616 Content-MD5。
Content-Type 表示请求内容的类型，如”application/octet-stream”，也可以为空。
Date 表示此次操作的时间，且必须为GMT格式，如”Sun, 22 Nov 2015 08:16:38 GMT”。
CanonicalizedOSSHeaders 表示以 x-oss- 为前缀的HTTP Header的字典序排列。
CanonicalizedResource 表示用户想要访问的OSS资源。
 */
    pub fn get_auth_headers(&self) -> HeaderMap {
        use crate::utils;
        let mut headers: HeaderMap = HeaderMap::new();
        // headers.insert(key: K, val: T)

        
        let content_md5 = "";
        let now_gmt = utils::get_now_gmt();
        let content_type = "application/json";
        let resource = utils::get_resource(&self.config.bucket_name, "", "");
        // VERB, Content-MD5, Content-Type, Date, CanonicalizedOSSHeaders, CanonicalizedResource
        let sign_str = format!("{}\n{}\n{}\n{}\n{}{}", "GET", content_md5, content_type, now_gmt.clone(), "", resource);
        let authorization_str = format!("OSS {}:{}", self.config.access_key_id, utils::content_sha1(&self.config.access_key_secret, &sign_str));
        
        headers.insert(OSSHeaders::ContentMD5.as_str(), content_md5.parse().unwrap());
        headers.insert(OSSHeaders::ContentType.as_str(), content_type.parse().unwrap());
        headers.insert(OSSHeaders::Date.as_str(), now_gmt.parse().unwrap());
        headers.insert(OSSHeaders::Host.as_str(), self.config.endpoint.parse().unwrap());
        headers.insert(OSSHeaders::Authorization.as_str(), authorization_str.parse().unwrap());
        headers.insert(OSSHeaders::UserAgent.as_str(), "rust-sdk-client/0.1.0".parse().unwrap());
        headers
    }

}

