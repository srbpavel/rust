extern crate easy_config;

mod config;
mod content;
mod handler;
mod handler_content_toml_config_struct;
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

    //HANDLER
    run(config).await
}
