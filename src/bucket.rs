// Copyright © Shaipe 
//! 
//! 
//! 

use crate::client::AliClient;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path};
use regex::Regex;

// mod service;
// use service::put_object;
#[derive(Debug, Clone)]
pub struct BucketContent {
    pub key: String,
    pub last_modified: String,
    pub etag: String,
    pub size: i64
}

impl BucketContent {

    pub fn new() -> Self {
        BucketContent {
            key: String::new(),
            last_modified: String::new(),
            etag: String::new(),
            size: 0
        }
    }

    
}





/// 上传单个对象到oss中
/// param1: 本地文件对象路径
#[allow(dead_code)]
pub fn put_object(file_path: &str) -> Result<String, std::io::Error> {
    let p = Path::new(file_path);
    if !p.exists(){
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "file not found"));
    };

    let mut file = match File::open(file_path) {
        Err(why) => {return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("couldn't create {}", why)));}
        Ok(file) => file,
    };

    let mut buf = Vec::new();
    match file.read_to_end(&mut buf){
        Ok(_) => {},
        Err(why) =>{
            return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("couldn't read {}", why)));
        } 
    }

    // 获取文件扩展名
    let file_extension = p.extension().and_then(std::ffi::OsStr::to_str).unwrap().to_owned();
    let re = Regex::new(r"gif|jpg|bmp|jpeg|png|ico$").unwrap();

    //content_type
    let content_type=match re.is_match(&file_extension){
        true=>format!("image/{}",file_extension),
        _=>String::from("text/plain")
    };

    // 根据file_path处理成带顶级/的路径
    let key = if file_path.starts_with("./"){
        format!("{}", file_path.replace("./", "/"))
    } else if file_path.starts_with("/"){
        file_path.to_owned()
    } else { 
        format!("/{}", file_path)
    };

    let mut c = AliClient::new();
    let res = c.do_request(http::Method::PUT, buf, content_type.as_str(), &key);
    match res {
        Ok(val) => {
            Ok(format!("{}", val.url()))
        },
        Err(e) => {return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("put object error {}", e)));}
    }
}




#[allow(dead_code)]
pub fn get_object(object_key: &str) {
    println!("{:?}", object_key);
}

