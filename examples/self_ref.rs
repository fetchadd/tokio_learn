#[derive(Debug)]
struct Person {
    self_ref: *const Person,
}

fn main() {
    let mut p1 = Person { self_ref: std::ptr::null(), };
    let mut p2 = Person { self_ref: std::ptr::null(), };

    p1.self_ref = &p1;
    p2.self_ref = &p2;

    println!("p1: address={:?}, self_ref={:?}", &p1 as * const _, p1.self_ref);
    println!("p2: address={:?}, self_ref={:?}", &p2 as * const _, p2.self_ref);

    std::mem::swap(&mut p1, &mut p2);
    println!();

    println!("p1: address={:?}, self_ref={:?}", &p1 as * const _, p1.self_ref);
    println!("p2: address={:?}, self_ref={:?}", &p2 as * const _, p2.self_ref);
}