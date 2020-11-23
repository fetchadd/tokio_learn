use std::ops::{Add, Deref};

fn main() {
    let h = "hello".to_string();
    let rh = & h;
    let hw = *rh;

    println!("{:?}", hw);
}