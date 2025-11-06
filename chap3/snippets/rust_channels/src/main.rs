// use std::time::Duration;

// use tokio::sync::oneshot;

// oneshot example
// async fn compute_something() -> String {
//     let _ = tokio::time::sleep(Duration::from_secs(5)).await;
//     "Helloworld".to_string()
// }
// using oneshot spsc channel with checking that if others tasks gets executed or not if our func goes into the non-blocking threadpool by tokio::spawn , and letting it wait for 5 sec and letting our func wait for 5 sec and everything gets executed ~5.1sec , as expected by the program.
// #[tokio::main]
// async fn main() {
// println!("Hello, world!");

//     let (tx,rx) = oneshot::channel();

// // spawning the func to the non-blocking threadpool
//     tokio::spawn( async move {
//         let data = compute_something().await;
//         tx.send(data).unwrap();
//     });


// // trying to call a blocking function , lets see if this executes within 5 sec or not
// // after : it did execute in ~5.2sec , 5sec of compute_func / 5sec of this / 5sec of below , ran concurrently and parallel on 1 thread with tokio::main(flavou=multi-thread,worker_threads=1)

//     tokio::task::spawn_blocking(|| {
//        let _ = tokio::time::sleep(Duration::from_secs(5));
//     });

//     println!("Waiting for 5 sec");
//     let _ = tokio::time::sleep(Duration::from_secs(5)).await;
//     let _rxdata = rx.await.unwrap();
//     println!("{}",_rxdata);

// }

// use std::time::Duration;

// use tokio::sync::mpsc;



// async fn some_compute(value: u64)-> String{
//     let _ = tokio::time::sleep(Duration::from_secs(2)).await;
//     format!("The value of computation is : {}",value)
// }

// // mpsc channel example
// #[tokio::main]
// async fn main() {
//     let (tx,mut rx) = mpsc::channel(100);


//     //using tx.send().await just bcz of its async and sending a lot of value concurrently and like in oneshot , no await is required as only a single value is passed at a time.
//     tokio::spawn(async move{
//         for i in 0..10{
//             let val = some_compute(i).await;
//             tx.send(val).await.unwrap();
//         }
//     });

//     // expectations: all 10 task will wait for 5 sec asynchronously , and the below will also wait async , so total time would be almost ~5.1sec , 
//     // results : mpsc channels tx.send await makes wait all tasks synchronously , so for 10 proc x 5 sec : 50sec , and the last execution of async task of 5 sec is used alongside with main block 5sec
//     let _ = tokio::time::sleep(Duration::from_secs(1)).await;

//     while let Some(res) = rx.recv().await {
//         println!("Received value: {}",res);
//     }
// }

// use tokio::sync::broadcast;

// #[tokio::main]
// async fn main(){
//     let (tx,mut rx1) = broadcast::channel(16);
//     let mut rx2 = tx.subscribe();

//     tokio::spawn(async move{
//         let mut val = rx1.recv().await.unwrap();
//         println!("Value1: {}",val);
//         assert_eq!(val,10);
//         val = rx1.recv().await.unwrap();
//         println!("Value1: {}",val);
//         assert_eq!(val,20);
//     });

//     tokio::spawn(async move{
//         let mut val = rx2.recv().await.unwrap();
//         println!("Value2: {}",val);
//         assert_eq!(val,10);
//         val = rx2.recv().await.unwrap();
//         println!("Value2: {}",val);
//         assert_eq!(val,20);
//     });

//     tx.send(10).unwrap();
//     tx.send(20).unwrap();

// }

use std::sync::Arc;

use tokio::sync::Mutex;


//using mutexes between threads
#[tokio::main]
async fn main(){
    let data1 =  Arc::new(Mutex::new(0));
    let data2 = Arc::clone(&data1);

    // both vars above have the value 0 , and will use tokio::spawn to spin up an async thread to update a single value , but for that we have to clone it for the thread , only the arc is cloned not the mutex , mutex is reference with it.

    let handle = tokio::spawn(async move {
        let mut lock = data2.lock().await;
        *lock+=1;
    });
    {
        let mut lock = data1.lock().await;
        *lock+=1;
    }
    handle.await.unwrap();
    let lock = data1.lock().await;
    println!("{:?}",*lock);
}