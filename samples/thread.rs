use std::thread;


#[allow(dead_code)]
fn print_hello(c: u8) {
    println!("       thread -> {}",
             c,
    );
}


fn main() {
    println!("#Starting\n");

    for i in 1..10 {
        println!("cycle: {}", &i);

        let ttt: thread::JoinHandle<_> = thread::spawn(move || print_hello(i));

        // SAMOVOLNE
        //ttt;

        // POSTUPNE
        ttt.join();

        /*
        thread::spawn(|| {
            println!("     thread...");
        }
        );
        */
    }

    println!("\n#Stopping\n");
}
