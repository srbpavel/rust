extern crate easy_config;
extern crate serde;

mod config;
mod handler_video_toml_config_struct;
mod handler;
mod message;
mod video;
mod util;

use handler::run;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // COMMAND ARGS
    let config_file = util::prepare_config(std::env::args()).unwrap_or_else(|err| {
        eprintln!("\nEXIT: Problem parsing cmd arguments\nREASON >>> {}", err);
        std::process::exit(1);
    });
    
    // CONFIG
    let config = config::sample_config(&config_file);

    //TEST VIDEO STORAGE
    let storage = std::path::Path::new(&*config.static_dir);

    // at the very start we want to check dir + verify we can write there
    // flag hardcoded not via conf
    match util::verify_dir(&storage.to_path_buf(), true) {
        Ok(_) => {},
        Err(err) => {
            // to LOG later
            eprintln!("VERIFY STORAGE: {}", err);
            
            std::process::exit(1)
        },
    };
    
    //HANDLER
    run(config).await
}
