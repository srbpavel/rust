use std::io::Read;
use std::io::Write;
use std::fs::File;

use std::io::BufRead;
use std::io::BufReader;


fn write_message_to_file(mut fout: &File) {

    let data = format!("foookume is KinG\nand Bijac also\n");

    println!("#GOING to write data to file >>>\n   START\n{}\n   END", data);
    
    //match fout.write(b"foookume is KinG\nand Bijac also\n") {
    match fout.write(data.as_bytes()) {
        Ok(written) => {
            println!("\n{} bytes written to\n\n#FILE: {:?}\n",
                     written,
                     fout.metadata(),
            );
        }
        Err(error) => {
            println!("write error: {}", error);
        }
    }
}
 
fn create_hello_world_file(file_name: &str) {
    match File::create(file_name) {
        Ok(fout) => {
            println!("#NEW FILE: {}", file_name);
            write_message_to_file(&fout);
        }
        Err(error) => {
            println!("file create error: {}", error);
        }
    }
}
 
fn read_one_byte(mut fin: &File) -> bool {
    //let mut buffer = vec![0];
    let mut buffer = [0; 1]; // fin.read size as per buffer size
    //let mut buffer = [0; 2]; // by two bytes
 
    match fin.read(&mut buffer) {
        Ok(size) => {
            if size > 0 {
                //println!("Read: '{}' = {}", buffer[0] as char, buffer[0]);
                // /*
                println!("Read: '{}' = {:?}",
                         buffer
                         .iter()
                         .map(|c| (*c as char).to_string())
                         .collect::<Vec<_>>()
                         //.join("")
                         .concat()
                         ,

                         buffer,
                );
                // */
            } else {
                println!("Read: ENDe");
            }
            
            size > 0
        }
        Err(error) => {
            println!("file read error: {}", error);
            false
        }
    }
}
 
fn main() {
    //
    let filename = "foookume.txt";

    create_hello_world_file(filename);
 
    let fin = File::open(filename).unwrap();

    // 
    let reader = BufReader::new(&fin);

    println!("#READ_LINES");
    for line in reader.lines() {
        match line {
            Ok(content) => {
                println!(" LINE: {}", content);
            }
            Err(error) => {
                println!("stdin read error: {}", error);
            }
        }
    }

    //
    let mut read_string = "".to_string();
    let mut fin_again = File::open(filename).unwrap();
    match fin_again.read_to_string(&mut read_string) {
        Ok(_) => println!("\n#READ_TO_STRING: {:?}\n", read_string),
        Err(_) => ()
    }
    
    // /*
    let fin_once_more = File::open(filename).unwrap();
    println!("\n#READ_per_BYTES");
    while read_one_byte(&fin_once_more) {
    }
    // */
}
