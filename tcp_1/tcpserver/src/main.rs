///TCP_SERVER
use std::net::TcpListener;
use std::io::{Read,
              Write,
};

/*
///
fn handle_connection(stream: TcpStream) {
}
*/

///
fn main() {
    // socket server
    let connection_listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    println!("running on port 3000");

    // here we wait for incomming connections
    // https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.incoming
    for stream in connection_listener.incoming() {
        //ok we have new connection -> Result<TcpStream,Error>
        let mut stream = stream.unwrap();
        /*
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => { /* connection failed */ }
        }
        */
        
        println!("connection established");

        // 
        let mut buffer = [0; 1024];

        // read incomming
        // https://doc.rust-lang.org/std/net/struct.TcpStream.html
        // here the .read() method is available due Trait Read
        // this pull bytes from source into buffer and returns bytes count
        stream
            .read(&mut buffer)
            .unwrap();

        println!("receive: <{}>",
                 &buffer
                 //&buffer[..10]
                 .iter()
                 //.inspect(|b| println!("{b}"))
                 // 0 is not zero but NULL
                 // dec / hex / bin / html / char / desc
                 // 0 00 000)0000 &#0; NUL Null
                 //.filter(|b| *b!=&b'0' )
                 .filter(|b| *b!=&0 )
                 .map(|b| (*b as char).to_string())
                 .collect::<Vec<_>>()
                 .concat()

        );
        
        // echo same data back
        stream
            .write(&mut buffer)
            .unwrap();
    }
}
