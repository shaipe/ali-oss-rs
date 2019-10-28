
use crate::client::AliClient;

pub fn get_buckets(profix: &str, marker: &str, max_keys: i64) -> String {
    let mut c = AliClient::new();
    c.do_request(http::Method::GET);
    "".to_owned()
}