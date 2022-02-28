extern crate easy_config;
extern crate serde;

// EASY_CONFIG          
mod config;
mod handler_video_toml_config_struct;

mod handler;
use handler::run;

mod message;
mod video;

const CONFIG_FILENAME: &str = "/home/conan/soft/rust/handler_video/src/handler_video_config.toml";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // CONFIG
    let config = config::sample_config(CONFIG_FILENAME);
    
    //HANDLER
    run(config).await
}

