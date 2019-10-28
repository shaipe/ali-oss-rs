extern crate ali_oss;
use ali_oss::config::{OSSConfig, set_config};
use ali_oss::service;

fn main() {
    let cnf: OSSConfig = OSSConfig::new("config.conf");
    set_config(cnf);
    // println!("{:?}", cnf);
    let s = service::get_buckets("", "", 100);
    println!("{:?}", s);
}