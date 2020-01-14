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
    let path=r#"/userfiles/supplier880/20200114/b7acc3d08a274b5086f198d68b802cae.png"#;
    // bucket::del_object(path);
    let mut v:Vec<String>=Vec::new();
    v.push(path.to_owned());
    bucket::del_mult_object(&v);
}