use std::env;
use std::fs;
use std::process;
use std::error::Error;


fn main() {
    let args: Vec<String> = env::args().collect();

    //let config = Config::new(&args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("EXIT: Problem parsing arguments\nREASON >>> {}", err); //err RETURN_MSG from Config::new
        process::exit(1);
    });

    
    if let Err(e) = read_config(config) {
        println!("ERROR reading file: {}", e);

        process::exit(1);
    }
}


fn read_config(config: Config) -> Result<(), Box<dyn Error>> {
    // sice namem PANIC! ale nevypisuju filename parametr jako pro .unwrap_or_else(|err|
    let data = fs::read_to_string(&config.filename)?;
    /*
    let data = fs::read_to_string(&config.filename)
        .expect(&format!("ERROR reading file: {}", &config.filename)); // zatim nevim proc &format!
    */

    println!("\ndata [{f}]:\n{d}",
             d=data,
             f=config.filename);

    Ok(())
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
            return Err("not enough arguments") // ERR_MSG
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
