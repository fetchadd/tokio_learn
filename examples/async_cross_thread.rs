use tokio::time::Duration;
use std::thread;
use rand::Rng;

#[derive(Debug)]
struct Person {
    name: String,
    self_ref1: *const Person,
    self_ref2: *const Person,
}


unsafe impl Send for Person {}

async fn test(mut p: Person, t: u64) {
    let old_id =  thread::current().id();

    p.self_ref2 = &p as * const _;

    let _ = tokio::time::sleep(Duration::from_secs(t)).await;

    println!(
        "{}:  old thread:{:?}, current thread {:?}, before spawn self_ref={:?}, before await self_ref={:?}  after await addr={:?}\n",
        p.name, old_id, thread::current().id(), p.self_ref1, p.self_ref2, &p as *const _
    );
}

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    let n = 5;
    let mut tasks= Vec::with_capacity(n);

    for i in 0..n {
        let mut p = Person {
            name: format!("t{}", i),
            self_ref1: std::ptr::null(),
            self_ref2: std::ptr::null(),
        };

        p.self_ref1 = &p;

        let sleep_time = rand::thread_rng().gen_range(1, 5);
        tasks.push(tokio::spawn(test(p, sleep_time)));
    }

    for task in tasks {
        let _ = task.await;
    }
}