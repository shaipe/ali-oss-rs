extern crate ali_oss;
use ali_oss::config::{OSSConfig, set_oss_config};
use ali_oss::bucket;

fn main() {
    let cnf: OSSConfig = OSSConfig::new("config.conf");
    set_oss_config(cnf);

    //获得
    //let s = bucket::get_buckets("", "", 100);

    //上传
    //bucket::put_object(r#"E:\rust\ali-oss-rs\QQ图片20190718094751.jpg"#);

    //删除
    //bucket::del_object(r#"/picture/QQ图片20190718094751.jpg"#);
    let mut v:Vec<String>=Vec::new();
    let path=r#"/picture/QQ图片20190718094751.jpg"#;
    v.push(path.to_owned());
    bucket::del_mult_object(&v);
}