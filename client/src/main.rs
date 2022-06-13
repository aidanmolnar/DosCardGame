
use std::{thread, time};

fn main() {
    println!("client");

    thread::sleep(time::Duration::from_millis(10000));

    println!("sleeped");
}