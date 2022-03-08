use serde::{Serialize, Deserialize};

type MyStr = Box<str>; 

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TomlConfig {
    // ROOT
    pub name: MyStr,
    pub secure: MyStr,
    pub host: MyStr,
    pub server: MyStr,
    pub port: u64,
    pub curl_limit_rate: MyStr,
    pub video_group: MyStr,
    pub upload_path: MyStr,
    pub player_path: MyStr,
    pub video_dir: MyStr,
    pub sample_limit_start: i8,
    pub sample_limit_end: i8,
    pub html_path: MyStr,
    pub html_template: MyStr,
    pub video_tag: MyStr,
    pub player_width: MyStr,
    pub content_type: MyStr,
    pub flag: Flag,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Flag {
    pub debug_config: bool,
    pub debug_template: bool,
}
