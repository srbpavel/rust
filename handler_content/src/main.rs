extern crate easy_config;
//extern crate serde;

mod config;
mod handler_content_toml_config_struct;
mod handler;
mod content;
mod util;
//mod status;
//mod error;

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

    /*
    println!("\nCPU get: {} / phy: {}",
             num_cpus::get(),
             num_cpus::get_physical(),
    );
    */

    //HANDLER
    run(config).await
}
