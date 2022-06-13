use std::{thread, time};

fn main() {
    println!("server");
    thread::sleep(time::Duration::from_millis(10000));

    println!("sleeped");
}