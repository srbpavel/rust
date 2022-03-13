use futures::{stream,
              StreamExt,
};
//use futures::channel::oneshot;
use async_std::task;
use std::{time::Duration,
          sync::mpsc::{
              channel,
              //Sender,
              //Receiver,
          },
          thread::spawn,
};
use rand::{thread_rng,
           Rng,
};


/// some concurent task
async fn run_task(sequence: u8,
                  sleep_limit: u64) -> Result<String, Box<dyn std::error::Error>> {

    let sleep_ms = generate_num(sleep_limit);
    
    task::sleep(
        Duration::from_millis(
            sleep_ms * 100
        )
    ).await;
    
    println!("{sequence} -> {sleep_ms}");

    Ok(
        format!("{sequence} -> {sleep_ms}")
    )
}


///
fn generate_num(limit: u64) -> u64 {
    let mut rng = thread_rng();
    rng.gen_range(0..limit)
}


///
fn mighty_vec() {
    let (sender, receiver) = channel();
    let hugevec = vec![0; 1_000_000];
    let mut newvec = vec![];
    let mut handle_vec = vec![];

    for i in 0..10 {
        print!("{i} -> ");
        
        let sender_clone = sender.clone();

        // 100_000
        let mut work: Vec<u8> = Vec::with_capacity(hugevec.len() / 10);

        work
            .extend(
                &hugevec[i*100_000..(i+1)*100_000]
            );

        let handle = spawn(move || {
            for number in work.iter_mut() {
                *number += 1;
            };

            sender_clone.send(work).unwrap();
        });

        handle_vec.push(handle);
    }
    
    for handle in handle_vec {
        handle
            .join()
            .unwrap();
    }

    while let Ok(results) = receiver.try_recv() {
        newvec
            .push(results);
    }

    let newvec = newvec
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>();
    
    println!("{:?}, {:?}, total length: {}",
             &newvec[0..10],
             &newvec[newvec.len()-10..newvec.len()],
             newvec.len()
    );
    
    for number in newvec {
        if number != 1 {
            panic!();
        }
    }
}

///
#[async_std::main]
async fn main() -> std::io::Result<()> {
    let count: u8 = 10;
    let sleep_limit: u64 = 20;
    
    //STREAM
    println!("#STREAM");

    stream::iter(0..count)
        .for_each_concurrent(20, |number| async move {

            let sleep_ms = generate_num(sleep_limit);

            task::sleep(
                Duration::from_millis(
                    sleep_ms * 100
                )
            ).await;
                
            println!("{} -> {}",
                     &number,
                     sleep_ms,
            );
        })
        .await;

    //JOIN
    println!("#JOIN");

    let v: Vec<u8> = (0..count).collect();
    
    let all_tasks: Vec<_> = v
        .into_iter()
        .map(|i|
             run_task(
                 i,
                 sleep_limit,
             )
        )
        .collect();

    let cmd_results = futures::future::join_all(all_tasks).await;

    cmd_results
        .iter()
        .for_each(|r|
                  println!("{r:?}")
        );

    // CHANNEL
    mighty_vec();

    /*
    //let (sender, receiver): (Sender<i32>, Receiver<i32>) = channel();
    let (sender, receiver) = channel();
    let sender_clone = sender.clone();
    let mut handle_vec = vec![];
    let mut result_vec = vec![];
    
    handle_vec.push(
        std::thread::spawn(move|| { // move sender in
            sender.send("Send a &str this time").unwrap();

        })
    );
    
    handle_vec.push(
        std::thread::spawn(move|| { // move sender_clone in
            sender_clone.send("And here is another &str").unwrap();
        })
    );
    
    handle_vec
        .iter()
        .for_each(|_|
                  result_vec.push(
                      receiver
                          .recv()
                          .unwrap()
                  )
        );

    println!("{:?}", result_vec);
    */

    Ok(())
}
