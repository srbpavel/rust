use std::future::Future;
use std::pin::Pin;
use std::task::{
    Context,
    Poll,
};
use std::thread::sleep;
use std::time::Duration;


struct ReadFileFuture {}

impl Future for ReadFileFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>,
            //_cx: &mut Context<'_>) -> Poll<Self::Output> {
            cx: &mut Context<'_>) -> Poll<Self::Output> {

        println!("Tokio! Stop polling me");

        // informs Tokio runtime async task is now ready to be
        // scheduled for execution again
        // --> poll future again
        cx
            .waker()
            .wake_by_ref();

        /*
        Poll::Pending
        // if without waker()
        // happens only once -> this register task to Waker
        // handle to Waker is stored in Contex
        */

        Poll::Ready(
            String::from("Hello, there from file 1")
        )
    }
}


#[tokio::main]
async fn main() {
    println!("Hello before reading file!");

    let h1 = tokio::spawn(
        async {
            let future1 = ReadFileFuture {};
            
            //future1.await
            println!("{:?}", future1.await);
        }
    );

    let h2 = tokio::spawn(
        async {
            let file2_contents = read_from_file2().await;
            
            println!("{:?}", file2_contents);
        }
    );

    let _ = tokio::join!(h1, h2);
}


// function that simulates reading from a file
fn read_from_file2() -> impl Future<Output = String> {
    async {
        sleep(Duration::new(2, 0));
        // --> after sleep we have Poll:Ready
        
        println!("{:?}", "Processing file 2");
        
        String::from("Hello, there from file 2")
    }
}

