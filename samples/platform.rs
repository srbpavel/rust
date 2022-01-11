use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
enum Platform {
    Linux,
    MacOS,
    Windows,
    Unknown,
}


impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Platform::Linux => write!(f, "Linux"),
            Platform::MacOS => write!(f, "macOS"),
            Platform::Windows => write!(f, "Windows"),
            Platform::Unknown => write!(f, "unknown"),
        }
    }
}


fn main() {
    let platform: String = Platform::Unknown.to_string();

    println!("{}", platform);
}
