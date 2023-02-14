
use actix_web::web::Data;
use actix_web::{main, App, HttpResponse, HttpServer, Responder, web};
use std::sync::Mutex;


async fn counter_get(counter: Data<Mutex<u32>>) -> impl Responder {
    let mut local_counter = match counter.lock() {
        Ok(c) => c,
        Err(_) => return HttpResponse::BadRequest().body("Mutex error")
    };
    if let Some(val) = local_counter.checked_add(1) {
        *local_counter = val;
    }
    println!("Counter is {}", local_counter);
    HttpResponse::Ok().body(format!("Now counter is {}", local_counter))
}

async fn counter_delete(counter: Data<Mutex<u32>>) -> impl Responder {
    let mut local_counter= match counter.lock() {
        Ok(c) => c,
        Err(_) => return HttpResponse::BadRequest().body("Mutex error")
    };
    *local_counter = 0;
    println!("Counter is {}", local_counter);
    HttpResponse::Ok().body("Counter reset")
}

#[main]
async fn main() -> std::io::Result<()> {
    let counter: Data<Mutex<u32>> = Data::new(Mutex::new(0));
    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone( &counter))
            .route("/counter", web::get().to(counter_get))
            .route("/counter", web::delete().to(counter_delete))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
