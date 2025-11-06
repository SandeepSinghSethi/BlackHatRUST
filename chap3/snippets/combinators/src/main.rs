use std::{collections::HashMap, vec};

fn main() {
    // println!("Hello, world!");

    let x:u64 = vec![1,2,3,4,5,6,7,8,9,10].into_iter().map(|x| x*x).sum();

    println!("{}",x);

    // hashmaps

    let mut hmap = HashMap::new();

    hmap.insert(String::from("hello"), String::from("World"));
    hmap.insert(String::from("wadu"), String::from("hekk"));

    for (k,v) in hmap.into_iter(){
        println!("Key: {} , Value: {}",k,v);
    };

    let fromiter = vec![(1,2),(3,4),(5,6)].into_iter();

    let _ :HashMap<u64,u64> = HashMap::from_iter(fromiter);

    // collect
    let c = vec![1,2,3,4,5].into_iter();
    let _ : Vec<i64>= c.collect();

    // filter
    let f = vec![-2,-1,0,1,2,3].into_iter();
    let f2 :Vec<i32>= f.filter(|x:&i32| x.is_positive()).collect();
    println!("f2: {:?}",f2);

    // inspect
    let i = vec![-2,-1,0,1,2,3,4,5].into_iter();
    let i2:Vec<i32> = i
        .inspect(|x| println!("Before inspect : {}",x))
        .filter(|x:&i32| x.is_positive())
        .inspect(|x| println!("After inspect: {}",x))
        .collect();
    println!("i2: {:?}",i2);


    //reduce
    let r = vec![1,2,3,4,5,6,7,8,9,10].into_iter();
    let r2 = r.reduce(|acc,x| acc + x).unwrap();
    println!("r2 : {}",r2);


    //flatten
    let fl = vec![vec![1,2,3,4,5],vec![6,7,8,9,10]].into_iter();
    let fl2:Vec<i32> = fl.flatten().collect();
    println!("fl2 : {:?}",fl2);


    //chain , chains two vec into one
    let c1 = vec![1,2,3,4,5].into_iter();
    let c2 = vec![6,7,8,9,10].into_iter();

    let c3 : Vec<i32>= c1.chain(c2).collect();
    println!("c3 : {:?}",c3);


    // filter_map : filter+map functinality
    let fm = vec!["Hello","World","!"].into_iter();
    let fm2 :Vec<String>= fm
        .filter_map(|x| {
            if x.len() > 2{
                Some(String::from(x))
            }
            else{
                None
            }
        })
        .collect();
    println!("fm2: {:?}",fm2);


    // combinators

    let a = vec![
            "1",
            "2",
            "-1",
            "4",
            "-4",
            "100",
            "invalid",
            "Not a number",
            "",
        ];

    let only_positive_num :Vec<i64>= a
        .into_iter()
        .filter_map(|x| x.parse::<i64>().ok())
        .filter(|x| x>&0)
        .collect();
    println!("{:?}",only_positive_num);


    // 
}
