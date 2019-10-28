use reqwest::{Client, RequestBuilder};
use crate::base64;
use md5::digest::FixedOutput;
extern crate sha1;
extern crate hmacsha1;
use hmacsha1::hmac_sha1;
use md5::{Md5, Digest};



use http::Method;

use crate::auth::Auth;
use crate::utils::get_now_gmt;
use crate::consts::OSSHeaders;
use crate::config::{get_config, OSSConfig};


pub struct AliClient{
    // 请求的客户端对象
    client: Client,
    // 基础配置对象
    config: OSSConfig
}


impl AliClient{

    pub fn new() -> Self {
        AliClient{ 
            client: Client::new(),
            config: get_config()
        }
    }

    pub fn do_request(&mut self, method: Method){
        let cnf = self.config.clone();
        let au = Auth::new(cnf.clone());
        // println!("{:?}", self.config);
        let url = format!("{}://{}", cnf.scheme, cnf.endpoint);
        let req: RequestBuilder = self.client.request(method, &url);
        let headers: reqwest::header::HeaderMap = au.get_auth_headers();
        
        println!("{:?}", headers);
        let res = req.headers(headers).send();
        println!("{:?}", res);
        println!("{:?}", res.unwrap().text());
    }
}   
