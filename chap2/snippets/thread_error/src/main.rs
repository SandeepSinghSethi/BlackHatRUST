use std::thread;

fn main(){
    let mut my_vec: Vec<i64> = Vec::new();

    thread::spawn(move || {
        add_to_vec(&mut my_vec);
    });

    my_vec.push(24);
    // compiler wont let us compile as it produces a race condition in the above code,  when trying to enter data on the same container at the same time.
}

fn add_to_vec(vec: &mut Vec<i64>){
    vec.push(42);
}
