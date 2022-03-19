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

    // this return result unless we send something else
    let join_result = tokio::join!(h1, h2);

    println!("{join_result:?}\ntime elapsed: {:?}",
             Instant::now() - start,
    );

    // just to see duration diff NEW vs FROM
    println!("\nduration:");
    // pass fn + delay

    /*
    //let dur_new = |delay: u64| -> Duration { Duration::new(delay, 0) };
    let dur_new = |delay| Duration::new(delay, 0);
    //let dur_from_m = |delay| Duration::from_millis(delay);
    let dur_from_m = |delay| Duration::from_millis(delay*1000);
    let dur_from_s = |delay| Duration::from_secs(delay);
    */

    /*
    let dur_new = Dur {
        dur: |delay| Duration::new(delay, 0)
    };

    let dur_s = Dur {
        dur: |delay| Duration::from_secs(delay)
    };

    let dur_m = Dur {
        dur: |delay| Duration::from_millis(delay)
    };

    println!("new: {:?}\nsec: {:?}\nmillis: {:?}",
             (dur_new.dur)(2),
             (dur_s.dur)(2),
             (dur_m.dur)(2000),
    );
    */

    /*
    sleep_s(
        (Dur::new(
            |delay| Duration::from_secs(delay))
         .dur)(2)
    );

    sleep_s(
        (Dur::new(
            |delay| Duration::from_millis(delay))
         .dur)(2000)
    );

    sleep_s(
        (Dur::new(
            |delay| Duration::new(delay, 0))
         .dur)(2)
    );
    */

    [
        (Dur::new(
            |delay| Duration::from_secs(delay))
         .dur)(2),
        (Dur::new(
            |delay| Duration::from_millis(delay))
         .dur)(2000),
        (Dur::new(
            |delay| Duration::new(delay, 0))
         .dur)(2),
    ]
        .iter()
        .for_each(|f|
                  sleep_s(*f)
        );
        
    /*
    println!("{:?}",
             (Dur::new(
                 |delay| Duration::from_secs(delay))
              .dur)(2)         
    );
    */
   
    /*
    [dur_from_s,
     dur_from_m,
     dur_new,
    ]
        .iter()
        .for_each(|f|
                  sleep_f(*f, 2)
        );
    */ 

    /*
    [
        (dur_from_s, 2),
        (dur_from_m, 2000),
        (dur_new, 2),
    ]
        .iter()
        .for_each(|(f,d)|
                  sleep_f(*f, *d)
                  // *Box::new(sleep_f(*f, *d))
        );
    */

    /*
    sleep_f(dur_from_s, 2);
    sleep_f(dur_from_m, 2*1000);
    sleep_f(dur_new, 2);
    */
}

// as closures inside main
// /*
//from_millis
fn dur_from_m(delay: u64) -> Duration {
    Duration::from_millis(delay)
}

//from_secs
fn dur_from_s(delay: u64) -> Duration {
    Duration::from_secs(delay)
}


//new
fn dur_new(delay: u64) -> Duration {
    Duration::new(delay, 0)
}
// */

//sleep s
fn sleep_s(delay: Duration) {    

    let start = Instant::now();
    
    sleep(
        delay
    );
    
    println!("{:?}",
             Instant::now() - start,
    );
}

//sleep display
fn sleep_f(f: fn(u64) -> Duration,
           delay : u64) {
    
    let start = Instant::now();

    sleep(
        f(delay)
    );

    println!("{:?}",
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

struct Dur {
    pub dur: fn(u64) -> Duration,
}

impl Dur {
    fn new(dur: fn(u64) -> Duration) -> Self {
        Self {
            dur,
        }  
    }
}

/*
struct Cacher<T>
where
    T: Fn(u64) -> Duration,
{
    calculation: T,
    value: Option<u64>,
}


impl<T> Cacher<T>
where
    T: Fn(u64) -> Duration,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self,
             arg: u64) -> Duration {

        match self.value {
            Some(v) => dur_from_s(arg),
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
*/
