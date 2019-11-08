// Copyright
//! oss对接配置提供存入静态缓存中


use std::collections::HashMap; 
use std::sync::Mutex;
use std::io::prelude::*;
use serde::Deserialize;
use std::fs::File;

// OSS配置信息
#[derive(Debug, Clone, Deserialize)]
pub struct OSSConfig {
    pub endpoint: String,
    pub access_key_id: String,
    pub access_key_secret: String,
    pub scheme: String,
    pub bucket_name: String
}

/// OSS配置默认实现
impl Default for OSSConfig {
    fn default() -> Self {
        OSSConfig{
            endpoint: String::new(),
            access_key_id: String::new(),
            access_key_secret: String::new(),
            scheme: String::from("http"),
            bucket_name: String::new()
        }
    }
}

/// OSS配置新实现
impl OSSConfig{

    pub fn new(conf_path: &str) -> Self {
        // 给定默认值
        let file_path = if conf_path.len() == 0 {
            "oss.conf"
        }
        else {
            conf_path
        };

        // 打开配置文件
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("no such file {} exception: {}", file_path, e)
        };

        // 读取文件到字符串变量, read_to_string 需要使用: use std::io::prelude::*;
        let mut str_val = String::new();
        match file.read_to_string(&mut str_val) {
            Ok(s) => s,
            Err(e) => panic!("Error Reading file:{}", e)
        };

        // serde_json 需要引用
        let cnf: OSSConfig = serde_json::from_str(&str_val).unwrap();

        cnf
    }
}

// 默认加载静态全局
lazy_static! {
    pub static ref PROXY_CACHES: Mutex<HashMap<String, OSSConfig>> = Mutex::new(HashMap::new());
}

/// 设置代码配置到全局缓存变量
pub fn set_oss_config(proxy: OSSConfig){
    PROXY_CACHES.lock().unwrap().insert("oss_config".to_owned(), proxy);
}

/// 从全局变量中获取代理配置
pub fn get_oss_config() -> OSSConfig {
    let cache = PROXY_CACHES.lock().unwrap(); 
    let cnf = match cache.get("oss_config") {
        Some(val) => val.clone(),
        _ => OSSConfig{
            endpoint: String::new(),
            access_key_id: String::new(),
            access_key_secret: String::new(),
            scheme: String::from("http"),
            bucket_name: String::new()
        }
    };
    cnf
}