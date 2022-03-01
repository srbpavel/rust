extern crate easy_config;
extern crate serde;

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

    //TEST DIR
    let storage = std::path::Path::new(&*config.static_dir);
    
    if !storage.exists() {
        eprintln!("Error: video_storage directory does not exists: {}",
                  &config.static_dir,
        );

        std::process::exit(1)
    } else {
        match std::fs::File::create(
            storage.join("writer.log")
        ) {
            Ok(_) => {},
            Err(why) => {
                eprintln!("Error: write permission to: {}\nREASON >>> {:?}",
                          &config.static_dir,
                          why,
                );

                std::process::exit(1)
            },
        }
    }
    
    //HANDLER
    run(config).await
}
