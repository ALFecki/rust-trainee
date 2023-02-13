use actix_web::web::Data;
use actix_web::{delete, get, main, App, HttpResponse, HttpServer, Responder};
use std::cell::{Cell, RefCell};

#[get("/counter")]
async fn counter_get(counter: Data<Cell<usize>>) -> impl Responder {
    counter.set(counter.get() + 1);
    println!("Counter is {}", counter.get());
    HttpResponse::Ok().body(format!("Now counter is {}", counter.get()))
}

#[delete("/counter")]
async fn counter_delete(counter: Data<Cell<usize>>) -> impl Responder {
    counter.set(0);
    println!("Counter is {}", counter.get());
    HttpResponse::Ok().body("Counter reset")
}

#[main]
async fn main() -> std::io::Result<()> {
    let counter: Cell<usize> = Cell::new(0);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(counter.clone()))
            .service(counter_get)
            .service(counter_delete)
    }).workers(1)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
