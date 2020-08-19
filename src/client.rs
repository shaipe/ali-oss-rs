// Copyright © Shaipe
//! 请求客户端

use crate::config::{get_oss_config, OSSConfig};
use crate::consts::OSSHeaders;
use http::Method;
use reqwest::blocking::{Client, RequestBuilder};
use reqwest::header::HeaderMap;

/// 阿里云请求客户端
pub struct AliClient {
    // 请求的客户端对象
    client: Client,
    // 基础配置对象
    config: OSSConfig,
}

impl AliClient {
    /// 创建一个新客户端对象
    pub fn new() -> Self {
        AliClient {
            client: Client::new(),
            config: get_oss_config(),
        }
    }
    /// 执行请求
    pub fn do_request(
        &mut self,
        method: Method,
        content: Vec<u8>,
        content_type: &str,
        query_params: &str,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let cnf = self.config.clone();
        let source_path = match query_params.len() {
            0 => String::from("/"),
            _ => format!("{}", query_params),
        };
        let url = format!("{}://{}{}", cnf.scheme, cnf.endpoint, source_path);

        let req: RequestBuilder = self
            .client
            .request(method.clone(), &url)
            .body(content.clone());
        let m = match method {
            Method::PUT => "PUT",
            Method::DELETE => "DELETE",
            Method::POST => "POST",
            _ => "GET",
        };
        let headers: reqwest::header::HeaderMap =
            self.get_auth_headers(m, content, content_type, source_path.as_str());
        req.headers(headers).send()
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
    fn get_auth_headers(
        &self,
        method: &str,
        content: Vec<u8>,
        content_type: &str,
        source_path: &str,
    ) -> HeaderMap {
        use crate::utils;
        let mut headers: HeaderMap = HeaderMap::new();
        // headers.insert(key: K, val: T)

        let content_md5 = utils::content_md5(&content);
        let now_gmt = utils::get_now_gmt();
        let resource = utils::get_resource(&self.config.bucket_name, "", "", source_path);
        // VERB, Content-MD5, Content-Type, Date, CanonicalizedOSSHeaders, CanonicalizedResource
        let sign_str = format!(
            "{}\n{}\n{}\n{}\n{}{}",
            method,
            content_md5,
            content_type,
            now_gmt.clone(),
            "",
            resource
        );
        //  println!("sign_str={:?}",sign_str);
        let authorization_str = format!(
            "OSS {}:{}",
            self.config.access_key_id,
            utils::content_sha1(&self.config.access_key_secret, &sign_str)
        );
        headers.insert(
            OSSHeaders::ContentMD5.as_str(),
            content_md5.parse().unwrap(),
        );
        headers.insert(
            OSSHeaders::ContentType.as_str(),
            content_type.parse().unwrap(),
        );
        headers.insert(OSSHeaders::Date.as_str(), now_gmt.parse().unwrap());
        headers.insert(
            OSSHeaders::Host.as_str(),
            self.config.endpoint.parse().unwrap(),
        );
        headers.insert(
            OSSHeaders::Authorization.as_str(),
            authorization_str.parse().unwrap(),
        );
        headers.insert(
            OSSHeaders::UserAgent.as_str(),
            "rust-sdk-client/0.1.0".parse().unwrap(),
        );
        //println!("headers={:?}",headers);

        headers
    }
}
