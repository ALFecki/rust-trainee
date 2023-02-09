use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::task::Poll::{Pending, Ready};
use std::thread;
use tokio::sync::RwLock;
use tokio::time::Instant;
use tokio::time::Duration;

pub fn my_sleep(duration: Duration) -> Delay {
    Delay {
        duration,
        time_to_sleep: Duration::new(0, 0),
        waker: None
    }
}

pub struct Delay {
    duration: Duration,
    time_to_sleep: Duration,
    waker: Option<Waker>
}

impl Future for Delay {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("polled");
        let instant = Instant::now();
        if self.time_to_sleep != self.duration {
            let _ = self.time_to_sleep.checked_add(instant.elapsed());
            self.as_mut().waker = Some(cx.waker().clone());
            return Pending;
        } else {
            return Ready(());
        }
    }
}


async fn get_image_id(name: &str) -> usize {
    tokio::time::sleep(std::time::Duration::from_millis((name.len() as u64) * 100)).await;
    name.len()
}

async fn get_static_image() -> usize { 1 }

async fn task1(names: Vec<&'static str>) {
    let mut handlers: Vec<Pin<Box<dyn Future<Output = usize>>>>  = vec![];
    for name  in names {
        handlers.push(Box::pin(get_image_id(name)));
    }
    handlers.push( Box::pin(get_static_image()));


    for handler in handlers {
        let res = handler.await;
        println!("{res}")
    }
}

#[tokio::main]
async fn main() {
    // let names = vec!["first_name", "sname", "first_name_and_last_name"];
    // task1(names.clone()).await;
    println!("hello");
    // let mut sleep = my_sleep(Duration::from_secs(2));
    // sleep.await;

    let sleep = TimerFuture::new(Duration::from_secs(2));
    sleep.await;
    println!("yes");
}



pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    completed: bool,

    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("polled");

        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {

    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        let thread_shared_state = shared_state.clone();
            thread::sleep(duration);
            let mut shared_state_1 = thread_shared_state.lock().unwrap();
            shared_state_1.completed = true;
            if let Some(waker) = shared_state_1.waker.take() {
                waker.wake()
            }

        TimerFuture { shared_state }
    }
}
