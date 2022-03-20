use std::future::Future;
use std::pin::Pin;
use std::task::{Context,
                Poll,
};
use std::thread::sleep;
use std::time::{Duration,
                Instant,
};

///
/// https://doc.rust-lang.org/stable/book/ch13-01-closures.html#capturing-the-environment-with-closures
///
/// https://stackoverflow.com/questions/27831944/how-do-i-store-a-closure-in-a-struct-in-rust
///
/// https://doc.rust-lang.org/book/ch19-05-advanced-functions-and-closures.html
///
struct MyDuration {
    pub value: fn(u64) -> Duration,
}

impl MyDuration {
    // takes closure + value and return Duration
    //
    // let dur_s = Dur {
    //    dur: |delay| Duration::from_secs(delay)
    // };
    //
    // println!("new: {:?}",
    //          (dur_new.dur)(2),
    // );
    //
    // (MyDuration::new(
    //        |delay| Duration::from_secs(delay))
    //     .value)(2),
    //
    fn new(value: fn(u64) -> Duration) -> Self {
        Self {
            value
        }  
    }

    // below is instead closure, because like that it cannot be used via iteration
    // as different type
    //
    // let dur_from_s = |delay| Duration::from_secs(delay);
    //
    // fn dur_from_s(delay: u64) -> Duration {
    //     Duration::from_secs(delay)
    // }
    //
    fn via_secs(delay: u64) -> Duration {
        Duration::from_secs(delay)
    }
    
    fn via_millis(delay: u64) -> Duration {
        Duration::from_millis(delay)
    }
    
    fn via_new(secs: u64,
               nanos: u32) -> Duration {
        
        Duration::new(secs, nanos)
    }
}


struct AsyncTimer {
    expiration_time: Instant,
}

// https://doc.rust-lang.org/std/future/trait.Future.html
//
impl Future for AsyncTimer {
    type Output = String;

    // https://doc.rust-lang.org/std/pin/struct.Pin.html
    // pinned pointer
    //
    // https://doc.rust-lang.org/std/pin/index.html
    //
    fn poll(self: Pin<&mut Self>,
            cx: &mut Context<'_>) -> Poll<Self::Output> {

        let now = Instant::now();
        println!("now: {now:?}\nexp: {:?}",
                 self.expiration_time,
        );

        // new instant is higher then our 4sec expiration
        //
        if now >= self.expiration_time {
            println!("Hello, it's time for Future 1");

            // poll prepared so we are finished
            Poll::Ready(
                String::from("Future 1 has completed")
            )
        // new instant is lower, we wait
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

                // all done, so let's inform waker or there will be
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

        let future_result = future1.await;
        
        println!("future_1: {:?}", future_result);

        future_result
    });

    let h2 = tokio::spawn(async {
        let file2_contents = read_from_file2().await;

        let future_result = file2_contents;
        
        println!("future_2: {:?}", future_result);

        future_result
    });

    // this return result Ok() unless we send something else
    let join_result = tokio::join!(h1, h2);

    println!("{join_result:?}\ntime elapsed: {:?}",
             Instant::now() - start,
    );

    // just to see duration diff NEW vs FROM + Instant
    println!("\nduration:");

    [
        (MyDuration::new(|delay|
                         Duration::from_secs(delay)
        ).value)(2),
        
        MyDuration::via_secs(2),
        MyDuration::via_millis(2000),
        MyDuration::via_new(2, 0),
    ]
        .iter()
        .for_each(|f|
                  sleep_s(*f)
        );
}

/// simple debug for some instant experiment
fn sleep_s(delay: Duration) {    

    let start = Instant::now();
    
    sleep(
        delay
    );
    
    println!("{:?}",
             Instant::now() - start,
    );
}

/// 2seconds sleep, will finish first
fn read_from_file2() -> impl Future<Output = String> {
    async {
        sleep(Duration::new(2, 0));

        String::from("Future 2 has completed")
    }
}

