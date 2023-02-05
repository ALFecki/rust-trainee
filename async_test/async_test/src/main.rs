use futures::future::join_all;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use std::time::Duration;

async fn get_image_id(name: &str) -> i32 {
    tokio::time::sleep(Duration::from_millis((name.len() as u64) * 100)).await;
    name.len() as i32
}

async fn task1(names: Vec<&'static str>) {
    let mut handles = vec![];
    for name in names {
        let handle = get_image_id(name);
        // let handle = tokio::spawn( async {
        //     get_image_id(name).await
        // });
        handles.push(handle);
    }
    // for handle in handles {
    //     println!("{}", handle.await);
    // }
    let results = join_all(handles.into_iter()).await;
    for res in results {
        println!("{res}",);
    }
}

async fn task2(names: Vec<&'static str>) {
    let mut futures = FuturesUnordered::new();
    for name in names {
        let handle = tokio::spawn(async { get_image_id(name).await });
        futures.push(handle);
    }
    if let Some(Ok(result)) = futures.next().await {
            println!("Get data {result}");
    }
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let names = vec!["first_name", "sname", "first_name_and_last_name"];
    task1(names.clone()).await;
    task2(names).await;
    Ok(())
}
