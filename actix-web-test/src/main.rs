mod xml;

use crate::xml::Xml;
use actix_web::web::{Data, head, Json, Query};
use actix_web::{main, web, App, Either, HttpRequest, HttpResponse, HttpServer, Responder, ResponseError};

use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use actix_web::http::StatusCode;
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

#[derive(Deserialize)]
struct CustomAdd {
    counter: u32,
}

fn change_counters(incr: &AtomicU32, to_swap: &AtomicU32, to_add: u32) -> u32 {
    to_swap.swap(0, Ordering::SeqCst);
    incr.fetch_add(to_add, Ordering::SeqCst) + to_add
}

fn get_accept_header(req: &HttpRequest) -> Option<&str> {
    match req.headers().get("accept") {
        Some(accept) => accept.to_str().ok(),
        None => None,
    }
}

// fn create_response<T>(headers: HttpRequest, data: T) -> impl Responder
//     where
//     T: Serialize,
// {
//     if let Some(accept) = get_accept_header(&headers) {
//         if let Ok(content_type) = accept.parse::<mime::Mime>() {
//             return match content_type.type_() {
//                 mime::XML => Xml(data).customize(),
//                 _ => Json(data).customize()
//             }
//         }
//     }
// }

async fn counter_get(
    headers: HttpRequest,
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

    if let Some(accept) = get_accept_header(&headers) {
        if accept == "application/xml" {
            return Either::Left(Xml(counters));
        }
    };
    Either::Right(Json(counters))
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

    // println!("{}", call(test, 1));
}

// fn call<Func, Args, Res>(function: Func, args: Args) -> Res
// where
//     Func: CallFunction<Args, Res>,
//     Args: FromStr,
// {
//     function.call(args)
// }
//
// pub trait CallFunction<Args, Res> {
//     fn call(&self, args: Args) -> Res;
// }
//
// impl<Func, Arg1, Res> CallFunction<Arg1, Res> for Func
// where
//     Func: Fn(Arg1) -> Res,
// {
//     fn call(&self, args: Arg1) -> Res {
//         self(args)
//     }
// }
//
// impl<Func, Arg1, Arg2, Res> CallFunction<(Arg1, Arg2), Res> for Func
// where
//     Func: Fn(Arg1, Arg2) -> Res,
// {
//     fn call(&self, args: (Arg1, Arg2)) -> Res {
//         self(args)
//     }
// }
//
// fn test(arg: i32) -> i32 {
//     arg
// }
