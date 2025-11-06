use serde::{Serialize,Deserialize}; // 1.0.228
use core::fmt::Debug;

#[derive(Debug,Clone,Serialize,Deserialze)]
struct Point<T: Debug + Clone + Serialize + Deserialize>{
    x:T,
    y:T,
}

fn main() {
    println!("Hello, world!");
}