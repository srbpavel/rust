use std::env;
use std::fs;
use std::process;


fn main() {
    let args: Vec<String> = env::args().collect();

    //let config = Config::new(&args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("EXIT: Problem parsing arguments\nREASON >>> {}", err);
        process::exit(1);
    });

    
    // /*
    let data = fs::read_to_string(&config.filename)
        .expect("Something went wrong reading the file");

    println!("\ndata [{f}]:\n{d}",
             d=data,
             f=config.filename);
    // */
}


struct Config {
    query: String,
    filename: String
}


impl Config {
    // Result<OK Config, ERROR &str>
    fn new(args: &[String]) -> Result<Config, &str> { // Config {

        const ARG_COUNT: usize = 3;
        
        if args.len() < ARG_COUNT {
            //panic!("NOT ENOUGH ARGUMENTS");
            return Err("not enough arguments")
        }

        println!("cmd: {:?} /\n\n[0]: {}",
                 args,
                 &args[0]);

        let query = args[1].clone();
        let filename = args[2].clone();

        println!("[1]: {}\n[2]: {}\n",
                 query,
                 filename);
        
        //Config { query, filename }
        //Ok(Config { query, filename }) // return Config as RESULT;
        return Ok(Config { query, filename });
    }

}
