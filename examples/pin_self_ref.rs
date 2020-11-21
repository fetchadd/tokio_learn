use std::marker::PhantomPinned;
use std::pin::Pin;

#[derive(Debug)]
struct Person {
    self_ref: *const Person,
    _pin: PhantomPinned,
}

fn main() {
    let mut p1 = Person { self_ref: std::ptr::null(), _pin: PhantomPinned};
    let mut p2 = Person { self_ref: std::ptr::null(), _pin: PhantomPinned};

    p1.self_ref = &p1;
    p2.self_ref = &p2;

    println!("p1: address={:?}, self_ref={:?}", &p1 as * const _, p1.self_ref);
    println!("p2: address={:?}, self_ref={:?}", &p2 as * const _, p2.self_ref);

    // Pin<P>::new要求P是P:Deref<Target: Unpin>的，这里Person是!Unpin的，所以无法使用Pin::new
    //let pin1 = Pin::new(&mut p1);
    //let pin2 = Pin::new(&mut p2);

    let mut pin1 :Pin<Box<Person>> = Box::pin(p1);
    let mut pin2 = Box::pin(p2);

    // get_mut方法定义如下
    //
    // impl<'a, T: ?Sized> Pin<&'a mut T> {
    //    pub fn get_mut(self) -> &'a mut T {...}
    // }
    // 所以 pin1是没有get_mut方法的，要想获取对Person的mut ref，只能通过下面代码

    let mut_ref1: Pin<&mut Person> = pin1.as_mut();
    let mut_ref2: Pin<&mut Person> = pin2.as_mut();

    unsafe {
        let p1_mut_ref: &mut Person = mut_ref1.get_unchecked_mut();
        let p2_mut_ref = mut_ref2.get_unchecked_mut();
        std::mem::swap(p1_mut_ref, p2_mut_ref);
    }

    println!();


    let ref1 :&Person = pin1.as_ref().get_ref();
    let ref2 = pin1.as_ref().get_ref();

    println!("p1: address={:?}, self_ref={:?}", ref1 as * const _, ref1.self_ref);
    println!("p2: address={:?}, self_ref={:?}", ref2 as * const _, ref2.self_ref);

    let mut s = "hello".to_string();
    let ps = Pin::new(&mut s);

    ps.get_mut();
}