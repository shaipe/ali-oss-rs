
use crate::client::AliClient;
use quick_xml::events::Event;
use quick_xml::Reader;
use crate::bucket::BucketContent;
use std::str::from_utf8;

pub fn get_buckets(profix: &str, marker: &str, max_keys: i64) -> String {
    println!("{:?}, {:?}, {:?}", profix, marker, max_keys);
    let mut c = AliClient::new();
    let res = c.do_request(http::Method::GET);
    let xml = res.unwrap().text().unwrap();

    let mut rdr = Reader::from_str(&xml);
    rdr.trim_text(true);

    let mut buf = Vec::new();
    let mut contents: Vec<BucketContent> = Vec::new();
    let mut txt = String::new();
    let mut cnt = BucketContent::new();

    loop {
        
        match rdr.read_event(&mut buf).expect("unable to read xml event") {
            Event::Text(ref e) =>  txt = e.unescape_and_decode(&rdr).unwrap(),
            Event::End(ref e) => {
                let n = from_utf8(e.local_name())
                    .expect("unable to build str from local_name")
                    .to_string();
                match n.as_str() {
                    "ETag" => {
                        cnt.etag = txt;
                    },
                    "Key" => {
                        cnt.key = txt;
                    },
                    "LastModified" => {
                        cnt.last_modified = txt;
                    },
                    "Size" => {
                        cnt.size = txt.parse::<i64>().unwrap();
                    },
                    "Contents" => {
                        contents.push(cnt);
                        cnt = BucketContent::new();
                    },
                    _ => {}
                };                    
                txt = String::from("");
            },
            Event::Eof => break,
            _ => {}
        }
    }

    println!("{:?}", contents);

    "".to_owned()
}