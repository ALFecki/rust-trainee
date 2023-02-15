use actix_web::web::{Data, Query};
use actix_web::{main, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;

struct Counters {
    counter: AtomicU32,
    delete_counter: AtomicU32,
    custom_counters: Arc<Mutex<HashMap<String, AtomicU32>>>,
}

impl Default for Counters {
    fn default() -> Self {
        Counters {
            counter: AtomicU32::new(0),
            delete_counter: AtomicU32::new(0),
            custom_counters: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[derive(Deserialize, Debug)]
struct CustomCounterQuery {
    na: Option<String>,
    me: Option<String>,
}

fn change_counters(incr: &AtomicU32, to_swap: &AtomicU32) -> u32 {
    to_swap.swap(0, Ordering::SeqCst);
    incr.fetch_add(1, Ordering::SeqCst) + 1
}

async fn counter_get(
    counters: Data<Counters>,
    params: Query<HashMap<String, String>>,
) -> impl Responder {
    let temp = change_counters(&counters.counter, &counters.delete_counter);
    let mut response = format!("Now counter is {}, delete counter reset", temp);
    if params.len() < 4 {
        let mut name = String::new();
        response += ", custom counter ";
        let mutex = counters.custom_counters.clone();
        let mut map = mutex.lock().await;
        for param in params.0 {
            name.push_str(param.1.as_str());
        }
        map.entry(name.clone())
            .and_modify(|c| {
                c.fetch_add(1, Ordering::SeqCst);
            })
            .or_insert(AtomicU32::new(1));
        if let Some(val) = map.get_key_value(&name) {
            response += format!(" ({}) is {:?}", name, val.1).as_str()
        }
    }
    println!("{}", response);
    HttpResponse::Ok().body(response)
}

async fn counter_delete(counters: Data<Counters>) -> impl Responder {
    let temp = change_counters(&counters.delete_counter, &counters.counter);
    println!("Counter is reset");
    println!("Delete counter is {}", temp);

    HttpResponse::Ok().body(format!("Counter reset, delete counter is {}", temp))
}

#[main]
async fn main() -> std::io::Result<()> {
    let counters: Data<Counters> = Data::new(Counters::default());

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
