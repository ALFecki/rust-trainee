use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::Poll::{Pending, Ready};
use std::task::{Context, Poll};
use std::thread;
use time::Instant;
use tokio::time::Duration;

pub fn my_sleep(duration: Duration) -> Delay {
    Delay {
        duration,
        timer: None
    }
}

pub struct Delay {
    duration: Duration,
    timer: Option<Instant>
}

impl Future for Delay {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.timer.is_none() {
            self.timer = Some(Instant::now());
        }
        if let Some(timer) = self.timer {
            return if timer.elapsed() < self.duration {
                cx.waker().clone().wake();
                Pending
            } else {
                Ready(())
            }
        } else {
            panic!("Timer error");
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
