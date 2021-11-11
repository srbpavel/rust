use std::fs;
use std::error::Error;
use std::env;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct"; //SEARCH STRING
        // start with \ no new_line \n
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


pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    // changes String -> &str slice
    let query = query.to_lowercase(); //oproti me one pouzijou & az pri volani .contains(&query)

    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) { // changes String -> &str slice
            results.push(line);
        }
    }

    results
}


// <'a> lifetime
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //println!("\n### i am searching");

    let mut results = Vec::new();

    // MY_SHADOW // let query = &query.to_lowercase(); // QUERY TO lower_case
    
    for line in contents.lines() {
        //let search_result = line.contains(query);

        
        // MY LOWER_CASE // if line.to_lowercase().contains(query) { // DATA LINE TO lower_case
        if line.contains(query) { // DATA LINE TO lower_case
            results.push(line);
        }
        
        /*
        println!("line [{b}]: {l}",
                 l=line,
                 b=&search_result);
        */
    }

    /*
    println!("\nTrue_lines: {:?}\n",
                 results);
    */
    
    // vec![] // TEST FAILED as we return empty vector
    results // TEST OK
}


pub fn read_config(config: Config) -> Result<(), Box<dyn Error>> {
    // sice namem PANIC! ale nevypisuju filename parametr jako pro .unwrap_or_else(|err|
    let data = fs::read_to_string(&config.filename)?;
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
    
    for line in results {
        count += 1;
        //println!("{}", line);
        println!("[{i}] result_line: {l}",
                 l=line,
                 i=count);
    }
    
    Ok(())
}


pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}


impl Config {
    // Result<OK Config, ERROR &str>
    pub fn new(args: &[String]) -> Result<Config, &str> { // Config {

        const ARG_COUNT: usize = 3;
        
        if args.len() < ARG_COUNT {
            //panic!("NOT ENOUGH ARGUMENTS");
            return Err("not enough arguments") // ERR_MSG
        }

        // /* DEBUG
        println!("cmd: {:?} /\n\n[0]: {}",
                 args,
                 &args[0]);
        // */

        let query = args[1].clone();
        let filename = args[2].clone();

        // PARAM as faster changing
        let case_sensitive: bool = args[3].clone().parse().unwrap();

        /* ENV
        // $export CASE_INSENSITIVE=1
        // $unset CASE_INSENSITIVE
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        */

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

