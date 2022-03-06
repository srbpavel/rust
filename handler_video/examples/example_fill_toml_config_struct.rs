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
    //pub workers: usize,
    //pub log_format: MyStr,
    pub upload_path: MyStr,
    pub player_path: MyStr,
    pub video_dir: MyStr,
    pub sample_limit: i8,

    pub html_file: MyStr,
    pub html_template: MyStr,
    pub video_tag: MyStr,
    pub player_width: MyStr,
    
    pub flag: Flag,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Flag {
    pub debug_config: bool,
    pub debug_template: bool,
}
