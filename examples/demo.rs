extern crate ali_oss;
use ali_oss::config::{OSSConfig, set_oss_config};
use ali_oss::service;

fn main() {
    let cnf: OSSConfig = OSSConfig::new("config.conf");
    set_oss_config(cnf);

    //获得
    //let s = service::get_buckets("", "", 100);

    //上传
    //service::put_object(r#"E:\rust\ali-oss-rs\QQ图片20190718094751.jpg"#);

    //删除
    //service::del_object(r#"/picture/QQ图片20190718094751.jpg"#);
    let mut v:Vec<&str>=Vec::new();
    v.push(r#"/picture/QQ图片20190718094751.jpg"#);
    service::del_mult_object(v);
}