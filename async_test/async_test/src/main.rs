use std::time::Duration;
use tokio::sync::mpsc::channel;

async fn get_image_id(name: &str) -> i32 {
    tokio::time::sleep(Duration::from_millis((name.len() as u64) * 100)).await;
    name.len() as i32
}

async fn task1(names: Vec<&'static str>) {
    let mut handles = vec![];
    for name in names {
        let handle = tokio::spawn(async { get_image_id(name).await });
        handles.push(handle);
    }
    for handle in handles {
        if let Ok(res) = handle.await {
            println!("{res}");
        }
    }
}

async fn task2(names: Vec<&'static str>) {
    let (tx, mut rx) = channel(5);

    let mut handles = vec![];

    for name in names {
        let tx_cpy = tx.clone();
        let handle = tokio::spawn(async move {
            tx_cpy.send(get_image_id(name).await).await;
        });
        handles.push(handle);
    }
    if let Some(res) = rx.recv().await {
        println!("{}", res);
    }
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let names = vec!["first_name", "sname", "first_name_and_last_name"];
    task1(names.clone()).await;
    task2(names).await;

    Ok(())
}