use serde::{Serialize,Deserialize}; // 1.0.228
use core::fmt::Debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Point<T: Debug + Clone + Serialize> {
    x:T,
    y:T,
}

fn main() {
    let _ = Point{x:12,y:122};
    println!("Hello, world!");
}