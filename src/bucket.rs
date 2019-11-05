// Copyright
//! 
//! 
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

#[allow(dead_code)]
pub fn put_object(object_key: &str) {
    println!("{:?}", object_key);
    //put_object("");
}

#[allow(dead_code)]
pub fn get_object(object_key: &str) {
    println!("{:?}", object_key);
}

