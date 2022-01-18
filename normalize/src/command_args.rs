use std::{env,
          process,
};

#[derive(Debug, Clone)]
pub struct CmdArgs {
    pub simulate: bool,
    pub full_path: String,
}


impl CmdArgs {
    pub fn new(mut args: env::Args) -> Result<CmdArgs, &'static str> {
        // HARDCODED as config with debug_flag's not parsed yet
        println!("\n#COMMAND {:#?}",
                 args);

        const ARG_COUNT: usize = 2 + 1; // sum of struct CmdArgs members + 1 PROGRAM

        if args.len() < ARG_COUNT {
            return Err("not enough arguments")
        } else if args.len() > ARG_COUNT {
            return Err("too many arguments")
        }

        let _program = match args.next() { // FUTURE_USE

            Some(arg) => arg,

            None => return Err("should never fail"),
        };

        let simulate = match args.next() {

            Some(arg) => arg
                .to_lowercase()
                .parse::<bool>()
                .unwrap_or_else(|err| {
                    eprintln!("\nEXIT: SIMULATE argument not true/false\nREASON: >>> {}", err);
                    process::exit(1);
                }),

            None => {
                eprintln!("no SIMULATE cmd_argument");
                process::exit(1);
            }
        };

        
        let full_path = match args.next() {

            Some(arg) => arg,

            None => return Err("no [full_path] cmd_argument"),
        };

        /* ENV
        // terminal: $ export CASE_INSENSITIVE=1
        // terminla: $ unset CASE_INSENSITIVE
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
         */

        return Ok(CmdArgs {simulate,
                           full_path,
        });
    }
}
