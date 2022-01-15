/*
use std::process;

// Struct for TOML config
mod toml_config_struct;
use toml_config_struct::{TomlConfig};

// local crate
use easy_config::{read_toml_config};
*/

mod example_easy_config;
mod toml_config_struct;


fn main() {
    // EXAMPLE >>> EASY_CONFIG
    example_easy_config::sample();
}
