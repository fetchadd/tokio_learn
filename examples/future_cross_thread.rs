use std::future::Future;
use std::task::{Poll, Context};
use std::thread;
use std::pin::Pin;
use tokio::time::{Duration, Sleep};
use rand::Rng;

#[derive(Debug)]
struct Person {
    name: String,
    sleep: Sleep,
    self_ref1: *const String,
    self_ref2: *const String,
}


unsafe impl Send for Person {}

impl Future for Person {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let p = self.get_mut();

        (*p).self_ref2 = (&(p.name)) as * const _;

        let old_id =  thread::current().id();

        {

            let sleep =  Pin::new(&mut ((*p).sleep)).poll(cx);

            if sleep.is_pending() {
                return Poll::Pending;
            }
        }

        println!(
            "{}:  old thread:{:?}, current thread {:?}, before spawn self_ref={:?}, before await self_ref={:?}  after await addr={:?}\n",
            p.name, old_id, thread::current().id(), p.self_ref1, p.self_ref2, (&(p.name)) as * const _
        );


        Poll::Ready(())
    }
}


#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    let n = 5;
    let mut tasks= Vec::with_capacity(n);

    for i in 0..n {
        let sleep_time = rand::thread_rng().gen_range(1, 5);

        let mut p = Person {
            name: format!("t{}", i),
            sleep: tokio::time::sleep(Duration::from_secs(sleep_time as u64)),
            self_ref1: std::ptr::null(),
            self_ref2: std::ptr::null(),
        };

        p.self_ref1 = &p.name;

        tasks.push(tokio::spawn(p));
    }

    for task in tasks {
        let _ = task.await;
    }
}