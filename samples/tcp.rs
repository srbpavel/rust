use std::thread;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;


fn utf8_to_string(bytes: &[u8]) -> String {
    let vector: Vec<u8> = Vec::from(bytes);
    String::from_utf8(vector).unwrap()
}

 
fn handler(mut stream:TcpStream,
           c: u8) {
    println!("Accepted connection <- {}", c);

    let response_text = format!("RUST Server response -> {}\r\n", c);
    
    stream.write(response_text.as_bytes())
        .unwrap();

    let mut buffer = [0; 16];
 
    loop {
        match stream.read(&mut buffer) {
            Ok(size) => {
                println!("read: {} bytes", size);
                if size == 0 {
                    println!("no data to read?");
                    break;
                } else {
                    let response = utf8_to_string(&buffer[0..size]);
                    println!("read: {:?}: '{}'", &buffer[0..size], response);

                    match stream.write(&buffer[0..size]) {
                        Ok(_)  => {}
                        Err(_) => {
                            println!("write error");
                            break;
                        }
                    }
                }
            }
            
            Err(_) => {
                println!("read error");
                break;
            }
        }
    }
    println!("disconnected")
}
 
fn main() {
    let mut counter = 0;

    let listener = TcpListener::bind("127.0.0.1:1234").unwrap();

    println!("RUST server started");
    
    for tcp_stream in listener.incoming() {
        match tcp_stream {
            Ok(tcp_stream) => {
                counter += 1;
                
                thread::spawn(move || {
                    handler(tcp_stream, counter);
                });
            },
            
            Err(e) => {
                println!("connection failed: {}", e);
            }
        }
    }
}


/*

$ curl --http0.9 "http://localhost:1234"
Server response...
curl: (56) Recv failure: Connection reset by peer

*/
