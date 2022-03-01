extern crate easy_config;
extern crate serde;

mod config;
mod handler_video_toml_config_struct;
mod handler;
mod message;
mod video;
mod util;

use handler::run;

use std::{env,
          process,
};

//const ARG_COUNT: usize = 2;
//const CONFIG_FILENAME: &str = "/home/conan/soft/rust/handler_video/src/handler_video_config.toml";


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // COMMAND ARGS
    let config_file = util::prepare_config(env::args()).unwrap_or_else(|err| {
        eprintln!("\nEXIT: Problem parsing arguments\nREASON >>> {}", err);
        process::exit(1);
    });
    
    // CONFIG
    //let config = config::sample_config(CONFIG_FILENAME);
    let config = config::sample_config(&config_file);

    //TEST VIDEO STORAGE
    let storage = std::path::Path::new(&*config.static_dir);

    // at the very start we want to test we can write
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
