use std::thread::sleep;
use std::time::Duration;


#[tokio::main]
async fn main() {
    println!("Hello before reading file!");

    let h1 = tokio::spawn(async {
        let file1_contents = read_from_file1().await;
        println!("{:?}", file1_contents);
    });
    
    let h2 = tokio::spawn(async {
        let file2_contents = read_from_file2().await;
        println!("{:?}", file2_contents);
    });
    
    let jjj = tokio::join!(h1, h2);

    println!("{jjj:?}");
}


// function that simulates reading from a file
async fn read_from_file1() -> String {
    println!("{:?}", "start 1");

    sleep(Duration::new(4, 0));

    println!("{:?}", "Processing file 1");

    String::from("Hello, there from file 1")
}


// function that simulates reading from a file
async fn read_from_file2() -> String {
    println!("{:?}", "start 2");

    sleep(Duration::new(2, 0));

    println!("{:?}", "Processing file 2");

    String::from("Hello, there from file 2")
}
