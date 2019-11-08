// Copyright (c) Shaipe 
//! Licensed under the Apache License, Version 2.0 (the "License")
//!
//!

use crate::bucket::BucketContent;
use crate::client::AliClient;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::fs::File;
use std::io::prelude::*;
use std::str::from_utf8;
use regex::Regex;

/// 获取bucket列表
pub fn get_buckets(profix: &str, marker: &str, max_keys: i64) -> String {
    println!("{:?}, {:?}, {:?}", profix, marker, max_keys);
    let mut c = AliClient::new();
    let res = c.do_request(http::Method::GET, vec![], "text/plain", "");
    let xml = res.unwrap().text().unwrap();

    let mut rdr = Reader::from_str(&xml);
    rdr.trim_text(true);

    let mut buf = Vec::new();
    let mut contents: Vec<BucketContent> = Vec::new();
    let mut txt = String::new();
    let mut cnt = BucketContent::new();

    loop {
        match rdr.read_event(&mut buf).expect("unable to read xml event") {
            Event::Text(ref e) => txt = e.unescape_and_decode(&rdr).unwrap(),
            Event::End(ref e) => {
                let n = from_utf8(e.local_name())
                    .expect("unable to build str from local_name")
                    .to_string();
                match n.as_str() {
                    "ETag" => {
                        cnt.etag = txt;
                    }
                    "Key" => {
                        cnt.key = txt;
                    }
                    "LastModified" => {
                        cnt.last_modified = txt;
                    }
                    "Size" => {
                        cnt.size = txt.parse::<i64>().unwrap();
                    }
                    "Contents" => {
                        contents.push(cnt);
                        cnt = BucketContent::new();
                    }
                    _ => {}
                };
                txt = String::from("");
            }
            Event::Eof => break,
            _ => {}
        }
    }

    println!("{:?}", contents);

    "".to_owned()
}

/// 上传单个对象到oss中
/// param1: 本地文件对象路径
pub fn put_object(path_file: &str) -> String {
    let mut f = File::open(path_file).unwrap();
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).unwrap();
    //以\\分隔
    let v: Vec<&str> = path_file.split('\\').collect();
    //文件名称
    let name=v[v.len() - 1];
    //以.分隔
    let extend_v:Vec<&str>=name.split('.').collect();
    //key
    let key = format!("/picture/{}", name);
    //文件扩展
    let file_extend=extend_v[extend_v.len()-1];
    let re = Regex::new(r"gif|jpg|bmp|jpeg|png|ico$").unwrap();
    //content_type
    let content_type=match re.is_match(file_extend){
        true=>format!("image/{}",file_extend),
        _=>String::from("text/plain")
    };
    let mut c = AliClient::new();
    let res = c.do_request(http::Method::PUT, buf, content_type.as_str(), key.as_str());
    let xml = res.unwrap().text().unwrap();
    print!("{:?}", xml);
    xml
}

/// 从oss中删除单个对象
/// param1: 文件对象路径
pub fn del_object(key: &str) -> String {
    let mut c = AliClient::new();
    let res = c.do_request(http::Method::DELETE, Vec::new(), "", key);
    let xml = res.unwrap().text().unwrap();
    print!("{:?}", xml);
    xml
}

/// 删除多个文件对象
/// param1: 对象路径集合 [&str]
pub fn del_mult_object(keys: Vec<&str>) -> String {
    let mut c = AliClient::new();
    let mut v:Vec<String>=Vec::new();
    for key in keys{
        v.push(format!("
    <Object> 
        <Key>{}</Key> 
    </Object>",key));
    }
    //Quiet=false 关闭简单响应模式
   let data= format!("<?xml version='1.0' encoding='UTF-8'?>
    <Delete>
    <Quiet>false</Quiet>
    {}
    </Delete>
    ",v.join(""));
    let res = c.do_request(http::Method::POST, data.as_bytes().to_vec(), "", "/?delete");
    let xml = res.unwrap().text().unwrap();
    print!("{:?}", xml);
    xml
}
