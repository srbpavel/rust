use std::future::Future;
use std::pin::Pin;
use std::task::{Context,
                Poll,
};
use std::thread::sleep;
use std::time::{Duration,
                Instant,
};


struct AsyncTimer {
    expiration_time: Instant,
}

impl Future for AsyncTimer {
    type Output = String;

    fn poll(self: Pin<&mut Self>,
            cx: &mut Context<'_>) -> Poll<Self::Output> {

        let now = Instant::now();
        println!("now: {now:?}\nexp: {:?}",
                 self.expiration_time,
        );

        // new instant is higher then out 4sec expiration
        //
        if now >= self.expiration_time {
            println!("Hello, it's time for Future 1");

            // poll prepared so we are finished
            Poll::Ready(
                String::from("Future 1 has completed")
            )
        // new instant is lower, so we wait
        } else {
            println!("Hello, it's not yet time for Future 1. Going to sleep");

            // Context provide access to &Waker
            // https://doc.rust-lang.org/std/task/struct.Context.html
            //
            // https://doc.rust-lang.org/std/task/struct.Waker.html
            // Waker is handle for waking up a task by notify executor
            //
            // we need to clone -> learn and understand why?
            let waker = cx
                .waker()
                .clone();

            // we need to make it live longer as it has anonym lifetime
            // as inside spawn(|| move {
            let expiration_time = self.expiration_time;

            std::thread::spawn(move || {
                let current_time = Instant::now();

                // we set another Instant and verify we are still before expiration
                if current_time < expiration_time {
                    let delay = expiration_time - current_time;

                    println!("delay: {delay:?}");

                    // we wait for calculated expiration
                    // here we can see diff
                    std::thread::sleep(delay);
                }

                // all done, so let's informa waker other there will be
                // infinite wait
                waker.wake();
            });

            // we are still pending but waker/context is informed via .wake()
            Poll::Pending
            }
        }
}


#[tokio::main]
async fn main() {
    let start = Instant::now();

    let h1 = tokio::spawn(async {
        let future1 = AsyncTimer {
            // https://doc.rust-lang.org/std/time/struct.Instant.html
            //
            // here we set 4second delay via Instant
            expiration_time: Instant::now() + Duration::from_millis(4000),
        };

        println!("future_1: {:?}", future1.await);
    });

    let h2 = tokio::spawn(async {
        let file2_contents = read_from_file2().await;

        println!("future_2: {:?}", file2_contents);
    });

    let _ = tokio::join!(h1, h2);

    println!("time elapsed: {:?}",
             Instant::now() - start,
    );
}


// 2seconds sleep, will finish first
fn read_from_file2() -> impl Future<Output = String> {
    async {
        sleep(Duration::new(2, 0));

        String::from("Future 2 has completed")
    }
}
