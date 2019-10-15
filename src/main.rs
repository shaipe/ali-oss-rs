extern crate ali_oss;

use ali_oss::OSSConfig;
use ali_oss::content_md5;

fn main() {
    let c: OSSConfig = OSSConfig::new();
    println!("{:?}", c);
    println!("{:?}", content_md5("0123456789"));
    println!("Hello, world!");
}
