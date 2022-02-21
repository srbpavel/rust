/// initial config properties
///
#[derive(Debug, Clone)]
pub struct InfluxConfig<'c> {
    pub name: &'c str,
    pub status: bool,

    pub secure: &'c str,

    pub server: &'c str,
    pub port: u16,

    pub bucket: &'c str,
    pub token: &'c str,
    pub org: &'c str,
    pub precision: &'c str,

    pub machine_id: &'c str,
    pub carrier: &'c str,
    pub flag_valid_default: bool,
}

impl <'c>InfluxConfig<'c> {
    pub fn new(name: &'c str,
               status: bool,
               secure: &'c str,
               server: &'c str,
               port: u16,
               bucket: &'c str,
               token: &'c str,
               org: &'c str,
               precision: &'c str,
               machine_id: &'c str,
               carrier: &'c str,
               flag_valid_default: bool) -> Self {
        
        Self {
            name,
            status,
            secure,
            server,
            port,
            bucket,
            token,
            org,
            precision,
            machine_id,
            carrier,
            flag_valid_default,
        }
    }

    pub fn default() -> Self {
        Self {
            name: "NAME",
            status: false,
            secure: "http", // https
            server: "localhost",
            port: 8086,
            bucket: "BUCKET",
            token: "TOKEN",
            org: "ORG",
            precision: "ms", // len()=13 -> 1645110902036
            machine_id: "MACHINE_ID",
            carrier: "CARRIER",
            flag_valid_default: false,
        }
    }
}
