use serde::{Serialize, Deserialize};
use std::collections::HashMap;

type MyStr = Box<str>; 

#[derive(Serialize, Deserialize, Debug)]
pub struct TomlConfig {
    // ROOT
    pub user: MyStr,

    pub name: MyStr,
    pub host: MyStr,
    pub work_dir: MyStr,

    pub mqtt_version: u32,
    
    pub service_type: MyStr,
    
    // STRUCT
    pub topics: Topics,
    pub flag: Flag,

    //iter via HASH key
    pub broker: HashMap<MyStr, Broker>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Topics {
    pub debug: bool,
    pub values: Vec<Topic>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Topic {
    pub status: bool,
    pub name: MyStr,
    pub body: MyStr,
    pub qos: i32,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Broker {
    pub machine: MyStr,
    pub client_id: MyStr,
    pub interval: u64,
        
    pub username: MyStr,
    pub password: MyStr,
        
    pub debug: bool,

    pub sub_lifetime: i8,
    pub sub_reconnect_delay: u64, 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Flag {
    pub debug_config: bool,
}
