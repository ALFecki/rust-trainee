use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::task::Poll::{Pending, Ready};
use tokio::time::Instant;
use tokio::time::Duration;

pub fn my_sleep(duration: Duration) -> Delay {
    Delay {
        duration,
    }
}

pub struct Delay {
    duration: Duration,
}

impl Future for Delay {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let check = self.duration;
        let instant = Instant::now();
        instant.checked_add(check);
        if instant.elapsed().as_secs() != check.as_secs() {
            return Pending;
        }
        Ready(())

    }
}






async fn get_image_id(name: &str) -> usize {
    tokio::time::sleep(std::time::Duration::from_millis((name.len() as u64) * 100)).await;
    name.len()
}

async fn get_static_image() -> usize { 1 }

// async fn task1(names: Vec<&'static str>) {
//     let mut handlers: Vec<_>  = vec![];
//     handlers = names.iter().map(|name| get_image_id(name)).collect();
//     handlers.push(get_static_image());
//
//
//     for handler in handlers {
//         let res = handler.await;
//         println!("{res}")
//     }
// }

async fn task2(names: Vec<&'static str>) {

    // let mut handlers = vec![];
    // let mut vec = vec![];
    // for name in names {
    //     handlers.push( tokio::spawn(async {get_image_id(name)}));
    // }
    // for handler in handlers {
    //     vec.push(handler.await);
    // }
    // select! {
    //     Some(Ok(val)) = vec.iter().next() => {
    //
    //     }
    // }





    // let mut set = JoinSet::new();
    // for name in names {
    //     set.spawn(async { get_image_id(name).await});
    // }
    // if let Some(Ok(res)) = set.join_next().await {
    //     println!("{res}");
    // }

}

#[tokio::main]
async fn main() {
    // let names = vec!["first_name", "sname", "first_name_and_last_name"];
    // task1(names.clone()).await;
    // task2(names).await;
    println!("hello");
    let sleep = my_sleep(Duration::from_secs(2));
    sleep.await;
    println!("yes");
}
