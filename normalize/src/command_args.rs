use clap::{Parser};

#[derive(Debug)]
#[derive(Parser)]
#[clap(
    version = "0.1.0",
    name="NORMALIZE",
    about = "\nReplace non alpha-numeric characters + remove diacritics + lowercase in given path or current directory files / descendant directories",
    author = "\nPavel SRB <prace@srbpavel.cz>")]
pub struct Args {
    #[clap(parse(from_os_str))]
    #[clap(
        default_value=".",
        help="working path / `pwd`",
        index=1)]
    pub path: std::path::PathBuf,

    #[clap(parse(try_from_str))]
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
        default_value="_",
        help="substitute char")]
    pub substitute_char: char,
}


pub fn read() -> Args {
    Args::parse()
}
