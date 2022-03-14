///TCP_CLIENT
use std::net::TcpStream;
use std::io::{Read, Write};
//use std::str;


///
fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();

    //let msg = "Hello";
    //let msg = "pAvEl";
    let msg = "foookume is KiNg";
    
    stream
        .write(msg
               .as_bytes()
        )
        .unwrap();

    // here we do not know how big data we receive
    // so we create larger buffer and filter later
    //let mut buffer = [0; 5];
    let mut buffer = [0; 1024];

    stream
        .read(&mut buffer)
        .unwrap();

    println!(
        "response from server: <{}>",
        //str::from_utf8(&buffer).unwrap(),
        &buffer
            .iter()
            //.inspect(|b| println!("{b}"))
            .filter(|b| *b!=&0 )
            .map(|b| (*b as char).to_string())
            .collect::<Vec<_>>()
            .concat()
    );
}
