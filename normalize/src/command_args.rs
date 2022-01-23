use clap::{Parser};
use std::path::Path;

// TRY to do the same with Build Patern
#[derive(Debug)]
#[derive(Parser)]
#[clap(
    version = "0.1.0",
    name="NORMALIZE",
    about = "\nRename files with non alpha-numeric characters + remove diacritics + lowercase in given path or current directory and descendant directories",
    author = "\nPavel SRB <prace@srbpavel.cz>")]
pub struct Args {
    #[clap(parse(from_os_str))] // path is possitional + default_value
    #[clap(
        default_value=".",
        help="working path / `pwd`",
        index=1)]
    pub path: std::path::PathBuf,

    #[clap(parse(try_from_str))] // this + bool type -> sets arg as required
    #[clap(
        short = 's',
        long = "simulate",
        help="dry run")]
    pub simulate: bool,

    #[clap(parse(try_from_str))]
    #[clap(
        short = 'r',
        long = "recursive",
        help="apply also for descendant dirs")]
    pub recursive: bool,

    #[clap(
        short = 'v',
        long = "verbose",
        help="display debug info")]
    pub verbose: bool,

    #[clap(
        name="SUBSTITUTE CHAR",
        short = 'c',
        long = "substitute",
        default_value="_", // default + char type -> optional
        help="substitute char")]
    pub substitute_char: char,
}


impl Args {
    pub fn default() -> Args {
        Args {
            path: Path::new("").to_path_buf(),
            simulate: false,
            recursive: false,
            verbose: false,
            substitute_char: '_',
        }
    }
}


pub fn read() -> Args {
    Args::parse()
}
