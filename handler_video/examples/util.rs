/// verify command args len
const ARG_COUNT: usize = 2;

/// config file as cmd arg
///
/// just single arg, no need for clap
///
pub fn prepare_config(mut cmd_args: std::env::Args) -> Result<String, &'static str> {
    
    if cmd_args.len() != ARG_COUNT {
        return Err("we want exactly one argumentnEXAMPLE: cargo run --example main /home/conan/soft/rust/handler_video/examples/example_fill_config.toml");
    }

    let _program = match cmd_args.next() {
        Some(arg) => arg,
        None => {
            return Err("should not fail unless wrong cargo call");
        }
    };
    
    let config_file = match cmd_args.next() {
        Some(arg) => arg,
        None => {
            return Err("Error: no CONFIG FILE");
        }
    };

    Ok(config_file)
}
