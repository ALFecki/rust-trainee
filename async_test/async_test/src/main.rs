use std::time::Duration;

async fn get_image_id(name: &str) -> i32 {
    tokio::time::sleep(Duration::from_millis((name.len() as u64) * 100)).await;
    name.len() as i32
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let _names = vec!["first_name", "sname", "first_name_and_last_name"];
    let a = tokio::spawn( async {
        get_image_id("first_name").await
    });

    let b = tokio::spawn( async {
        get_image_id("sname").await
    });
    let c = tokio::spawn( async {
        get_image_id("first_name_and_last_name").await
    });
    println!("{:?}", b.await);

    println!("{:?}", a.await);
    println!("{:?}", c.await);


    Ok(())
}
