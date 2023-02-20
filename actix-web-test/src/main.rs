mod utilities;
mod xml;

use crate::xml::Xml;
use actix_web::web::{Data, Json, Query};
use actix_web::{main, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use crate::utilities::{change_counters, get_accept_header};
use actix_web::body::EitherBody;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Default, Serialize)]
struct Counters {
    counter: AtomicU32,
    delete_counter: AtomicU32,
    #[serde(skip_serializing)]
    custom_counters: Arc<Mutex<HashMap<String, AtomicU32>>>,
}

#[derive(Deserialize, Debug)]
struct CustomCounterQuery {
    n: String,
    a: String,
    m: String,
    e: String,
}

struct ContentTypeResponse<T>
where
    T: Clone,
{
    data: T,
}

impl<T> Responder for ContentTypeResponse<T>
where
    T: Clone + Serialize,
{
    type Body = EitherBody<String>;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        let mut data_in_response = Json(self.data.clone()).respond_to(req);
        if let Some(accept) = get_accept_header(&req) {
            if let Ok(content_type) = accept.parse::<mime::Mime>() {
                match content_type.subtype() {
                    mime::XML => data_in_response = (Xml(self.data)).respond_to(req),
                    _ => data_in_response = Json(self.data).respond_to(req),
                }
            }
        }
        data_in_response
    }
}

#[derive(Deserialize)]
struct CustomAdd {
    counter: u32,
}

async fn counter_get(
    counters: Data<Counters>,
    params: Option<Query<CustomCounterQuery>>,
    counter: Option<Query<CustomAdd>>,
) -> impl Responder {
    let mut to_add = match counter {
        Some(val) => val.counter,
        None => 1,
    };
    if let Some(params) = params {
        let mut name = String::new();
        let mutex = counters.custom_counters.clone();
        let mut map = mutex.lock().await;

        name.push_str(&params.n);
        name.push_str(&params.a);
        name.push_str(&params.m);
        name.push_str(&params.e);

        map.entry(name.clone())
            .and_modify(|c| {
                c.fetch_add(to_add, Ordering::SeqCst);
            })
            .or_insert_with(|| AtomicU32::new(to_add));
        to_add = 1;
    }
    change_counters(&counters.counter, &counters.delete_counter, to_add);

    ContentTypeResponse { data: counters }
}

async fn counter_delete(counters: Data<Counters>) -> impl Responder {
    let temp = change_counters(&counters.delete_counter, &counters.counter, 1);
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
