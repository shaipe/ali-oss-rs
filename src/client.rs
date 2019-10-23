use reqwest::Client;
use crate::base64;
use md5::digest::FixedOutput;
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

pub struct AliClient{
    client: Client
}

impl AliClient{

    pub fn new() -> Self {
        AliClient{ 
            client: Client::new()
        }
    }


}
use md5::{Md5, Digest};
// 对签名的内容进行content-md5计算
pub fn content_md5(text: &str) -> String {

    // let mut m = Md5::new();
    // m.input_str(text);
    // // m.length_bytes = 128;
    // let xt = m.output_bits();
    // let yy = m.result_str();
    // println!("yy:; {:?}", yy);
    // let digest = md5::compute(text.as_bytes());
    // println!("{:?}",digest);
    // let new_sign = format!("{:x}", digest);
    // // pr
    // for n in 0..16 {
    //     let st = n * 2;
    //     let end = st + 2;
    //     let x = &new_sign[st..end];
    //     println!("{:?}", x.parse::<i32>().unwrap());
    // }
    let mut m = Md5::default();
    m.input(text.as_bytes());
    //m.fixed_result();
    //println!("ssss{:?}",m.buffer);
    let digest = m.fixed_result();
    println!("{:?}", base64::encode(&digest));
    base64::encode(&digest)
}
