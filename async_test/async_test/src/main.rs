use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::Poll::{Pending, Ready};
use std::task::{Context, Poll};
use std::thread;
use tokio::time::Duration;

pub fn my_sleep(duration: Duration) -> Delay {
    Delay {
        to_share: Arc::new(Mutex::new(Share {
            completed: false,
            duration,
        })),
    }
}

pub struct Delay {
    to_share: Arc<Mutex<Share>>,
}

pub struct Share {
    completed: bool,
    duration: Duration,
}

impl Future for Delay {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let share_cpy = self.to_share.clone();
        return if !share_cpy.lock().unwrap().completed {
            thread::spawn(move || {
                let mut share = share_cpy.lock().unwrap();
                thread::sleep(share.duration);
                share.completed = true;
            });
            cx.waker().clone().wake();
            Pending
        } else {
            Ready(())
        }
    }
}

#[tokio::main]
async fn main() {
    println!("hello");
    // tokio::time::sleep()
    let sleep = my_sleep(Duration::from_secs(5));
    println!("Something");
    sleep.await;
    println!("yes");
}
