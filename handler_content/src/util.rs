/// verify command args len
const ARG_COUNT: usize = 2;

/// config file as cmd arg
///
/// just single arg, no need for clap
///
pub fn prepare_config(mut cmd_args: std::env::Args) -> Result<String, &'static str> {
    
    if cmd_args.len() != ARG_COUNT {
        return Err("we want exactly one argument\n example:\n  $ cargo run /home/conan/soft/rust/handler_content/src/handler_content_config.toml\n  $ /home/conan/soft/rust/handler_content/target/debug/handler_content /home/conan/soft/rust/handler_content/src/handler_content_config.toml");
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
