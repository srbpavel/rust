use serde::{Serialize, Deserialize};


type MyStr = Box<str>; 

#[derive(Serialize, Deserialize, Debug)]
pub struct TomlConfig {
    // ROOT
    pub user: MyStr,

    pub name: MyStr,
    pub host: MyStr,
    pub work_dir: MyStr,
    
    // STRUCT
    pub broker: Broker,
    pub flag: Flag,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Broker {
    pub machine: MyStr,
    pub client_id: MyStr,
    pub interval: u64,
        
    pub username: MyStr,
    pub password: MyStr,
        
    pub debug: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Flag {
    pub debug_config: bool,
}
