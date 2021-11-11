use std::fs;
use std::error::Error;


pub struct Config {
    pub query: String,
    pub filename: String
}


impl Config {
    // Result<OK Config, ERROR &str>
    pub fn new(args: &[String]) -> Result<Config, &str> { // Config {

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


pub fn read_config(config: Config) -> Result<(), Box<dyn Error>> {
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
