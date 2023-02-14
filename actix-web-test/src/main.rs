use actix_web::web::Data;
use actix_web::{delete, get, main, App, HttpResponse, HttpServer, Responder, web};
use std::sync::{Arc, Mutex, RwLock};

// #[get("/counter")]
async fn counter_get(counter: Data<Mutex<u32>>) -> impl Responder {
    println!("get");

    let mut local_counter = counter.lock().unwrap();
    if let Some(val) = local_counter.checked_add(1) {
        *local_counter = val;
    }
    println!("Counter is {}", local_counter);
    HttpResponse::Ok().body(format!("Now counter is {}", local_counter))
}

// #[delete("/counter")]
// async fn counter_delete(counter: Data<Cell<u32>>) -> impl Responder {
//     counter.set(0);
//     println!("Counter is {}", counter.get());
//     HttpResponse::Ok().body("Counter reset")
// }

#[main]
async fn main() -> std::io::Result<()> {
    let counter: Data<Mutex<u32>> = Data::new(Mutex::new(0));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone( &counter))
            .route("/counter", web::get().to(counter_get))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
