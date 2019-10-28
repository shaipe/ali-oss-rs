use chrono::{NaiveDateTime,Local};

pub struct OSSHeader {
    authorization: String,
    content_length: i64,
    content_type: String,
    date: NaiveDateTime,
    host: String,
}

impl Default for OSSHeader {

    fn default() -> OSSHeader {
        OSSHeader{
            authorization: String::new(),
            content_length: 0i64,
            content_type: String::new(),
            date: Local::now().naive_local(),
            host: String::new()
        }
    }
}