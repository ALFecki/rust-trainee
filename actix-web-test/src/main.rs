use actix_web::web::Data;
use actix_web::{main, web, App, HttpResponse, HttpServer, Responder};
use std::sync::atomic::{AtomicU32, Ordering};

struct Counters {
    counter: AtomicU32,
    delete_counter: AtomicU32,
}

fn change_counters(incr: &AtomicU32, to_swap: &AtomicU32) -> u32 {
    to_swap.swap(0, Ordering::SeqCst);
    incr.fetch_add(1, Ordering::SeqCst) + 1
}

async fn counter_get(counters: Data<Counters>) -> impl Responder {
    let temp = change_counters(&counters.counter, &counters.delete_counter);
    println!("Counter is {}", temp);
    println!("Delete counter is reset");
    HttpResponse::Ok().body(format!("Now counter is {}, delete counter reset", temp))
}

async fn counter_delete(counters: Data<Counters>) -> impl Responder {
    let temp = change_counters(&counters.delete_counter, &counters.counter);
    println!("Counter is reset");
    println!("Delete counter is {}", temp);

    HttpResponse::Ok().body(format!("Counter reset, delete counter is {}", temp))
}

#[main]
async fn main() -> std::io::Result<()> {
    let counters: Data<Counters> = Data::new(Counters {
        counter: AtomicU32::new(0),
        delete_counter: AtomicU32::new(0),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&counters))
            .route("/counter", web::get().to(counter_get))
            .route("/counter", web::delete().to(counter_delete))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
