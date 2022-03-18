use std::thread::sleep;
use std::time::Duration;
use std::future::Future;

/*
fn read_from_file1() -> impl Future<Output=String> {

poll function returns enum -> 
Poll::Pending
Poll::Ready(val)

the upper is call via async executor


pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
*/


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

    let _ = tokio::join!(h1, h2);
}


// function that simulates reading from a file
fn read_from_file1() -> impl Future<Output=String> {
    async {
        sleep(Duration::new(4, 0));
        println!("{:?}", "Processing file 1");
        String::from("Hello, there from file 1")
    }
}


// function that simulates reading from a file
fn read_from_file2() -> impl Future<Output=String> {
    async {
        sleep(Duration::new(3, 0));
        println!("{:?}", "Processing file 2");
        String::from("Hello, there from file 2")
    }
}
