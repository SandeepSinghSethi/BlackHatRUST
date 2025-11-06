use std::{thread, time::Duration};

use futures::{stream, StreamExt};
use rand::{Rng};

// to prevent crashing the program , always try to prevent block the event loop i.e., executing a functions that tends to execute within 10ms-100ms , this way optimal flow event loop will function.

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    // println!("Hello, world!");
    stream::iter(0..200u64)
        .for_each_concurrent(20,|number| async move {
            let mut rng = rand::rng();
            let sleep_ms = rng.random_range(0..20);
            tokio::time::sleep(Duration::from_millis(sleep_ms)).await;

            let thread_id = thread::current().id();
            let curr_thread = thread::current();
            let thread_name = curr_thread.name().unwrap_or("unknown-thread");

            println!("Task : {} is executed on thread : {:?} with name : {}",number,thread_id,thread_name);
        })
        .await;
}
