
// OSS配置信息
#[derive(Debug, Clone)]
pub struct OSSConfig {
    pub endpoint: String,
    pub access_key_id: String,
    pub access_key_secret: String
}

impl OSSConfig {
    pub fn new() -> Self {
        OSSConfig{
            endpoint: "".to_owned(),
            access_key_id: "".to_owned(),
            access_key_secret: "".to_owned()
        }
    }
}