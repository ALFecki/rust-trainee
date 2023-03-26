extern crate core;
use std::future::Future;
use std::pin::Pin;
use std::task::Poll::{Pending, Ready};
use std::task::{Context, Poll};
use time::Instant;
use tokio::time::Duration;

pub fn my_sleep(duration: Duration) -> Delay {
    Delay {
        duration,
        timer: None,
    }
}

pub struct Delay {
    duration: Duration,
    timer: Option<Instant>,
}

impl Future for Delay {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let timer = match self.timer {
            None => Instant::now(),
            Some(t) => t,
        };
        if timer.elapsed() < self.duration {
            cx.waker().wake_by_ref();
            self.timer = Some(timer);
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
    let sleep = my_sleep(Duration::from_secs(2));
    println!("Something");
    sleep.await;
    println!("yes");
}
