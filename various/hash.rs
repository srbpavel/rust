use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::HashMap;

fn main() {

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    //println!("{:?}", scores.entry(String::from("Red")));
    println!("entry: {:?}", scores.entry(String::from("Blue")));
    scores.entry(String::from("Red")).or_insert(333);
    
    println!("{:?} / blue: {}", scores, scores["Blue"]);

    let mut map = HashMap::new();
    map.insert("mama", String::from("vaanda"));
    map.insert("tata", String::from("jirka"));

    println!("{:?}", map);

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
    ///////////////////////////////////////////////////

    if let Ok(lines) = read_lines("config.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
    
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
