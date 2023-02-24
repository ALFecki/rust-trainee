mod utilities;
mod xml;

use crate::utilities::{change_counters, get_accept_header};
use crate::xml::Xml;
use actix_web::body::EitherBody;
use actix_web::web::{Data, Json, Query};
use actix_web::{delete, get, main, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::io::ErrorKind::NotFound;

use actix_web::error::ContentTypeError::{ParseError, UnknownEncoding};
use actix_web::error::ReadlinesError::ContentTypeError;
use actix_web::http::header::ACCEPT;
use mime::{Mime, Name};
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
    T: Serialize,
{
    data: T,
}

impl<T> ContentTypeResponse<T>
where
    T: Serialize,
{
    fn get_content_type(&self, req: &HttpRequest) -> Result<Mime, &str> {
        if let Some(accept) = get_accept_header(req) {
            if let Ok(mime) = accept.parse::<Mime>() {
                return Ok(mime);
            }
        }
        Err("Mime is shit")
    }
}

impl<T> Responder for ContentTypeResponse<T>
where
    T: Serialize,
{
    type Body = EitherBody<String>;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        let mime = 'get_mime: {
            if let Some(accept) = get_accept_header(req) {
                if let Ok(mime) = accept.parse::<Mime>() {
                    break 'get_mime mime;
                }
            }
            return HttpResponse::from_error(ContentTypeError(UnknownEncoding))
                .map_into_right_body();
        };
        let mime = mime.subtype();
        match mime {
            mime::XML => Xml(self.data).respond_to(req),
            mime::JSON => Json(self.data).respond_to(req),
            _ => HttpResponse::from_error(ContentTypeError(ParseError)).map_into_right_body(),
        }
    }
}

#[derive(Deserialize)]
struct CustomAdd {
    counter: u32,
}

#[get("/counter")]
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

#[delete("/counter")]
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
            .service(counter_get)
            .service(counter_delete)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
