use serde::{Serialize, Deserialize};

type MyStr = Box<str>; 

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TomlConfig {
    // ROOT
    pub name: MyStr,
    pub host: MyStr,
    pub server: MyStr,
    pub port: u64,
    pub workers: usize,
    pub static_dir: MyStr,
    pub log_format: MyStr,
    pub flag: Flag,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Flag {
    pub debug_config: bool,
}
