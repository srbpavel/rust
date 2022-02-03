use clap::{App,
           AppSettings,
           Arg,
           ArgMatches,
};

use std::path::{Path,
                PathBuf,
};


const NAME: &str = "NORMALIZE";
const ABOUT: &str = "\n\
                     Rename FILES [not DIRS] with non alpha-numeric characters \
                     + remove diacritics + lowercase in given path \
                     or current directory and descendant directories\
                     ";

const USAGE: &str = "\n\
                     normalize DIR [OPTION].. \n\n\
                     normalize /temp/ -s -v -r -c=#\n\
                     [dry-run + verbose + recursive rename files in /temp/ with \
                     usage of # substitute]\
                     ";


// `pwd`
const DEFAULT_PATH: &str = ".";
const DEFAULT_CHAR: char = '_';


mod options {
    pub static PATH: &str = "path";
    pub static SIMULATE: &str = "simulate";
    pub static RECURSIVE: &str = "recursive";
    pub static VERBOSE: &str = "verbose";
    pub static SUBSTITUTE: &str = "substitute";
}


#[derive(Debug, Default)]
pub struct Settings {
    pub path: PathBuf,
    pub simulate: bool,
    pub recursive: bool,
    pub verbose: bool,
    pub substitute: char,
}


impl Settings {
    /// default values
    pub fn default() -> Settings {
        Settings {
            path: Path::new(&DEFAULT_PATH)
                .canonicalize() // relative -> full_path
                .unwrap()
                .to_path_buf(),
            
            simulate: false,
            recursive: false,
            verbose: false,
            substitute: DEFAULT_CHAR,
        }
    }

    /// fill with values from cmd args
    fn new(matches: ArgMatches) -> Settings {
        // start with DEFAULT and update as it go
        let mut settings = Settings { ..Settings::default() };

        // ALL bool FLAG's
        settings.simulate = matches.is_present(options::SIMULATE);
        settings.recursive = matches.is_present(options::RECURSIVE);
        settings.verbose = matches.is_present(options::VERBOSE);

        // PATH
        if let Some(p) = matches.value_of(options::PATH) {
            let path = Path::new(p);

            if path.exists() { settings.path = path
                               .canonicalize() // relative -> full_path
                               .unwrap()
                               .to_path_buf()
            } else {
                eprintln!("\nERROR: ARG path: <{path:?}> not valid -> we will stop");
                
                std::process::exit(1)
            };
        }
        
        // SUBSTITUTE_CHAR
        settings.substitute = parse_substitute_char(
            matches.value_of(options::SUBSTITUTE),
            &settings,
        );

        // UPDATED
        settings
    }
}


/// workflow for substitute char
///
/// use DEFAULT_CHAR if not provided
///
/// exit if provided but empty like -c= or -c=""
///
/// exit if not single char, but multiple as -c=---
///
fn parse_substitute_char(sub: Option<&str>,
                         settings: &Settings) -> char {

    match sub {
        Some(value) => {
            if settings.verbose {
                println!("substitute_char value: <{value}>");
            }
            
            // slice &[u8]
            let bytes = value.as_bytes();
            
            match bytes.len() {
                // CORRECT len
                1 => { 
                    let substitute = bytes[0] as char;
                    
                    if settings.verbose {
                        println!("char: '{substitute}'");
                    }
                    
                    substitute
                },
                
                // PROVIDED but EMPTY
                0 => {
                    println!("EXIT: empty char: '{}'", value);
                    
                    std::process::exit(1);
                },

                // TO MANY CHARS
                _ => {
                    println!("EXIT: too many chars: '{:}'", value);
                    
                    std::process::exit(1);
                },
            }
        },
        
        None => {
            
            // DEFAULT char
            settings.substitute
        },
    }
}


/// clap App properties
fn new_clap_app<'a>() -> App<'a> {

    App::new(NAME)
        .about(ABOUT)
        .override_usage(USAGE)
        .setting(AppSettings::InferLongArgs)
        .arg(
            Arg::new(options::PATH)
                // ONLY single path
                .multiple_occurrences(false)
                .takes_value(true)
                .index(1)
                .default_value(DEFAULT_PATH),
        )
        .arg(
            Arg::new(options::SIMULATE)
                .short('s')
                .long(options::SIMULATE)
                .help("dry run"),
        )
        .arg(
            Arg::new(options::RECURSIVE)
                .short('r')
                .long(options::RECURSIVE)
                .help("apply also for descendant dirs"),
        )
        .arg(
            Arg::new(options::VERBOSE)
                .short('v')
                .long(options::VERBOSE)
                .help("display debug info"),
        )
        .arg(
            Arg::new(options::SUBSTITUTE)
                .short('c')
                .long(options::SUBSTITUTE)
                .takes_value(true)
                // BECAUSE char's as: -c # or -c * BREAKS IT
                // -c=* or -c="*" works's OK
                .require_equals(true)
                // SET IN Settings::default()
                //.default_value("_")
                .help("substitute char"),
        )
}


/// main call to get Settings from cmd_args
pub fn get_settings() -> Settings {
    // CLAP App + ArgMatches
    let app = new_clap_app();
    let matches = app.get_matches();

    // CMD ARGS to Struct
    Settings::new(matches)
}
