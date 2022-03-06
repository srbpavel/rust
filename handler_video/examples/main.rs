extern crate easy_config;
extern crate serde;
//extern crate handler_video;

mod example_config;
mod example_fill_toml_config_struct;
//mod handler;
//mod video;
mod util;
//mod status;

//use handler::run;
//use util;

fn main() {
    // COMMAND ARGS
    let config_file = util::prepare_config(std::env::args()).unwrap_or_else(|err| {
        eprintln!("\nEXIT: Problem parsing cmd arguments\nREASON >>> {}", err);
        std::process::exit(1);
    });
    
    // CONFIG
    let config = example_config::sample_config(&config_file);

    println!("EXAMPLE_CONFIG: {:#?}",
             config,
    );
}

