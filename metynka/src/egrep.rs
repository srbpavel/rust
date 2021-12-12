use std::fs;
use std::error::Error;

use crate::settings::CmdArgs;


pub fn search_case_insensitive<'a>(query: &str, data: &'a str) -> Vec<&'a str> {
    // NOT CASE SENSITIVE
    
    data
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()

    /*
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in data.lines() {
        if line.to_lowercase().contains(&query) { // changes String -> &str slice
            results.push(line);
        }
    }

    results
    */
}


pub fn search_case_sensitive<'a>(query: &str, data: &'a str) -> Vec<&'a str> {
    // CASE SENSITIVE

    data
        .lines()
        .filter(|line| line.contains(query))
        .collect()
    
    /*
    let mut results = Vec::new();

    for line in data.lines() {
        // MY LOWER_CASE 
        if line.contains(query) { // DATA LINE TO lower_case
            results.push(line);
        }
        
    }

    // vec![] // TEST FAILED as we return empty vector
    results // TEST OK
    */
}


// EGREP tutorial
pub fn read_config(args: CmdArgs) -> Result<(), Box<dyn Error>> {
    /*
    let mut data = String::new();
    fs::File::open(&args.filename)?.read_to_string(&mut data)?;
     */
    
    let data = fs::read_to_string(&args.filename)?;
    
    let results = match args.case_sensitive {
        true => search_case_sensitive(&args.query, &data),
        false => search_case_insensitive(&args.query, &data)
    };

    let mut count: u8 = 0;
    let count_closure = |x: u8| -> u8 { x + 1 };

    println!("\n#EGREP:\nfile: {f}\nquery: \"{q}\"\ncase_sensitive: {cs}\n\nRESULTS:",
             f=&args.filename,
             q=&args.query,
             cs=args.case_sensitive);
    
    for line in results {
        count = count_closure(count); // INSTEAD count += 1;
        println!("[{i:?}]: {l}",
                 l=line.trim(),
                 i=count);
    }
    
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*; // GLOB

    /*
    #[test]
    fn panic_test() {
        panic!("### MAKE THIS TEST FAIL");
    }
    */

    
    #[test]
    fn one_result() {
        let query = "duct"; //SEARCH STRING
        // start's with \ no new_line \n
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
