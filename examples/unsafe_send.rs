use std::ptr::NonNull;
use std::thread;

struct UnSend {
    data: String,
    slice: NonNull<String>,
}

unsafe impl Send for UnSend {}

fn main() {
    let mut un_send = UnSend {
        data: "hello".to_string(),
        slice: NonNull::dangling(),
    };

    un_send.slice = NonNull::from(&un_send.data);

    let t = thread::spawn(move || {
        println!(
            "address in main {:?}, address in thread {:?}",
            un_send.slice.as_ptr(), &un_send.data as * const _
        );
    });

    let _ = t.join();
}