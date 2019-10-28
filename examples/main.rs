extern crate ali_oss;

// use ali_oss::OSSConfig;
use ali_oss::content_md5;
use ali_oss::content_sha1;
extern crate chrono;
use chrono::{DateTime, Utc,Local};

extern crate reqwest;
fn main() {
    let now: DateTime<Utc> = Utc::now();
    let format_time=now.to_rfc2822().to_string().replace("+0000","GMT");
    println!("{:?},{:?},{:?}",format_time,now.to_rfc2822(),Local::now().naive_utc());
    // let c: OSSConfig = OSSConfig::new();
    let md5_value=content_md5("0123456789");
    //println!("{:?}", c);
    //println!("{:?}", md5_value);
    let access_key_secret="";
    let access_key_id="";
    let data=format!("PUT\n{}\ntext/html\n{}\nx-oss-magic:abracadabra\nx-oss-meta-author:foo@bar.com\n/tempkm/",md5_value,format_time);
    let access_key_secret= content_sha1(&access_key_secret,&data);
    println!("data={:?}",data);
    let signature =format!("OSS {}:{}",access_key_id,access_key_secret);
    let client = reqwest::Client::new();
    let res = client.post("http://img.b2b321.366ec.net")
    .header("Signature", &signature)
    .header("Method", "PUT")
    .header("Content-Md5", &md5_value)
    .header("Content-Type", "text/html")
    .header("Date", format_time)
    .header("Host", "b2b321.366ec.net")
    .header("X-OSS-Magic", "abracadabra")
    .header("X-OSS-Meta-Author", "foo@bar.com")
    .body(format!("0123456789"))
    .send();
    println!("{:?}",res);
}
