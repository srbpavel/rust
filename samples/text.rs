#[allow(unused_imports)]
use std::env;

use std::collections::HashSet;
use std::hash::Hash;


macro_rules! power {
    ($base:expr) => (
        $base*$base
    );

    ($base:expr, $exponent:expr) => (
        ($base as i32).pow($exponent)
    );
}

macro_rules! trace {
    ($expression:expr) => (
        println!("{:?} = {}",
                 stringify!($expression),
                 $expression,
        );
    )
}

macro_rules! whoami {
    () => (
       "SpongeBoB"
    );

    /*
    () => (
        println!("{}\n",
                 "spongebob"
                 .to_string()
                 .to_uppercase(),
        );
    )
    */
}


fn print_static_str(message: &'static str) {
    println!("static str: {}", message);
}

fn print_str(message: & str) {
    println!("str: {}", message);
}

fn print_string(message: String) {
    println!("String: {}", message);
}

fn return_str() -> &'static str {
    "fookume"
}


//fn print_hashset(set: &HashSet<&str>) {
fn print_hashset<T: std::fmt::Display>(set: &HashSet<T>) {
    for item in set {
        println!("{}", item);
    }
}

fn print_hashset_string(set: &HashSet<String>) {
    for item in set {
        println!("{}", item);
    }
}


fn vec2set<T: Copy + Eq + Hash>(v: Vec<T>) -> HashSet<T> {
    v.iter().cloned().collect()
}


fn main() {
    // HashSet
    /*
    let set2: HashSet<&str> = vec!["tovarnik", "stevard", "podkoni", "inspektor"]
        .iter()
        .cloned()
        .collect();
    */
    //let set2 = vec2set(vec!["tovarnik", "stevard", "podkoni", "inspektor"]);
    let set2 = vec2set(vec![3,1,5,9,6,4,2]);

    // String
    let set3: HashSet<_> = vec!["tovarnik", "stevard", "podkoni", "inspektor", "podkoni"] //multiple occur -> only once in Set
        .iter()
        .map(|x| x.to_string())
        .collect();
 
    println!("#Set2:");
    print_hashset(&set2);
    println!("set2: {:?}", set2);

    println!("#Set3:");
    print_hashset_string(&set3);
    println!("set3: {:?}", set3);


    // MACRO
    //whoami!();
    println!("|{w:.>width$}|{w:_<width$}|{w:-^width$}|",
             w=whoami!(),
             width=20,
    );
    
    // ENV
    /*
    let mut vvv = env::vars()
        //.filter(|(x,_)| x!="LS_COLORS")
        //.map(|(x,y)| format!("{}: {}", x, y))
        .filter_map(|(x,y)| match x!="LS_COLORS" {
            true => Some(format!("{}: {}", x, y)),
            false => None,
        }
        )
        .collect::<Vec<_>>();//[0..=5]

    //vvv.sort();
    vvv.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    
    println!("{:#?}",
             //&vvv[vvv.len()-5..],
             vvv,
    );
    */

    println!("{}",
             power!(4),
    );
    
    println!("{}",
             //power!(2, 8),
             power!(16, 2),
    );
    
    trace!(2i32.pow(3));
    //_
    
    let message_str = return_str();
 
    print_str(message_str);

    print_static_str(message_str);

    let mut message_string = return_str().to_string();
    print_str(&message_string);

    message_string.push_str(" is KinG");
    print_str(&message_string);

    print_string(message_string);

    let one: String = "ik".to_string();
    let three: String = "paavel".to_string();

    //println!("{}", &one);
    //println!("{}", &three);

    //let ned = one + " ben " + &three + " srb";
    //println!("{}", &ned);
    
    let ned = format!("{} {} {} {}",
                      one,
                      "ben",
                      three,
                      "srb",
    );
    
    println!("{}", ned);

    let mut all_bytes = Vec::new();
    for byte in ned.as_bytes() {
        all_bytes.push(byte);
    }

    println!("all_bytes: {:?}", all_bytes);

    
    let mut all_chars = Vec::new();
    for char in ned.chars() {
        all_chars.push(char);
    }

    let slice_1: usize = 7;
    let slice_2: usize = 7;
    
    println!("all_chars: {:?}\nslice[{}][{}]: <{}>",
             all_chars,

             slice_1,
             slice_2,
             
             //all_chars[7..=all_chars.len()-1]
             //all_chars[7..all_chars.len()]
             &all_chars[slice_1..]
             .iter()
             .map(|&c| c.to_string().to_uppercase())
             .collect::<String>()
             .as_str()[slice_2..]
    );

    
    /*
    let mut message: &'static str = "foookin paavel";
    print_str(message);

    message = "bijac";
    print_str(message);
    print_str(message);
    */
}
