use std::fs;
use std::error::Error;
use std::env;
// use std::io::Read;


#[cfg(test)]
mod tests {
    use super::*; // GLOB

    #[test]
    fn panic_test() {
        panic!("### MAKE THIS TEST FAIL");
    }

    
    #[test]
    fn one_result() {
        let query = "duct"; //SEARCH STRING
        // start's with \ no new_line \n
        // tim padem se me vrati jen radek ktery obsahuje QUERY
        let contents = "\
Rust:
safe, fast, productive.
Pick three."; // DATA

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }


    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    
}


pub fn search_case_insensitive<'a>(query: &str, data: &'a str) -> Vec<&'a str> {
    data
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()

    /*
    // changes String -> &str slice
    let query = query.to_lowercase(); //oproti me pouzivaji & az pri volani .contains(&query)

    let mut results = Vec::new();

    for line in data.lines() {
        if line.to_lowercase().contains(&query) { // changes String -> &str slice
            results.push(line);
        }
    }

    results
    */
}


// <'a> lifetime
pub fn search<'a>(query: &str, data: &'a str) -> Vec<&'a str> {
    data
        .lines()
        .filter(|line| line.contains(query))
        .collect()
    
    /*
    let mut results = Vec::new();

    // MY_SHADOW // let query = &query.to_lowercase(); // QUERY TO lower_case
    for line in data.lines() {
        // MY LOWER_CASE // if line.to_lowercase().contains(query) { // DATA LINE TO lower_case
        if line.contains(query) { // DATA LINE TO lower_case
            results.push(line);
        }
        
    }

    // vec![] // TEST FAILED as we return empty vector
    results // TEST OK
    */
}


pub fn read_config(config: Config) -> Result<(), Box<dyn Error>> {
    // sice namem PANIC! ale nevypisuju filename parametr jako pro .unwrap_or_else(|err|

    let data = fs::read_to_string(&config.filename)?;

    /* TROSKU JINE VOLANI
    let mut data = String::new();
    fs::File::open(&config.filename)?.read_to_string(&mut data)?;
    */
    
    /*
    let data = fs::read_to_string(&config.filename)
        .expect(&format!("ERROR reading file: {}", &config.filename)); // zatim nevim proc &format!
    */

    // /* DEBUG
    println!("\n###DATA_START [{f}]:\n{d}\n###DATA_END\n",
             d=data,
             f=config.filename);
    // */

    /*
    let mut count: u8 = 0;
    for line in search(&config.query, &data) {
        count += 1;
        println!("[{i}] result_line: {l}",
                 l=line,
                 i=count);
     */

    let results = if config.case_sensitive {
        search(&config.query, &data)
    } else {
        search_case_insensitive(&config.query, &data)
    };

    let mut count: u8 = 0;
    //let count_closure = |x| x + 1;
    let count_closure = |x: u8| -> u8 { x + 1 };

    for line in results {
        //count += 1;
        count = count_closure(count);
        println!("[{i:?}] result_line: {l}",
                 l=line,
                 i=count);
    }
    
    Ok(())
}


pub struct Config {
    // do not forget to chance ARG_COUNT verification 
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}


impl Config {
    // Result<OK Config, ERROR &str>
    // pub fn new(args: &[String]) -> Result<Config, &str> { // Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> { // Config {

        const ARG_COUNT: usize = 4;
        
        if args.len() < ARG_COUNT {
            //panic!("NOT ENOUGH ARGUMENTS");
            return Err("not enough arguments") // ERR_MSG
        }

        //args.next(); // we ignore first call as there is program name
        let program = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string [keyword]"),
        };

        // /* DEBUG // cargo call is with relative path 'target/debug/ts' different then rustc
        println!("\ncmd: {:?} /\n\n[0]: {}",
                 args,
                 program);
        // */

        /*
        let query = args[1].clone();
        let filename = args[2].clone();
        // PARAM as faster changing
        let case_sensitive: bool = args[3].clone().parse().unwrap();
        // let case_sensitive = args[3].clone().parse::<bool>().unwrap();
        // let case_sensitive = String::from(args[3].clone()).parse().unwrap();
        // let case_sensitive = String::from(args[3].clone()).parse::<bool>().unwrap();
        */

        /* ENV
        // $export CASE_INSENSITIVE=1
        // $unset CASE_INSENSITIVE
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        */

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string [keyword]"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name [path]"),
        };

        let case_sensitive = match args.next() {
            Some(arg) => arg.parse::<bool>().unwrap(),
            None => return Err("Didn't get a case sensitive [bool]"),
        };
        
        
        // /* DEBUG
        println!("[1]: {}\n[2]: {}\n[3]: {}",
                 query,
                 filename,
                 case_sensitive);
        // */
        
        //Config { query, filename }
        //Ok(Config { query, filename }) // return Config as RESULT;
        return Ok(Config {query, filename, case_sensitive});
    }

}

