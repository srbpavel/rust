use futures::{stream,
              StreamExt,
};
use rand::{thread_rng,
           Rng,
};
use std::time::Duration;
use async_std::task;


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

            println!("{number} -> {sleep_ms}");
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
                  if r.is_err() {
                      println!("ERROR: {r:?}");
                  } else {
                      println!("{r:?}");
                  }
                  
        );
    
    Ok(())
}
