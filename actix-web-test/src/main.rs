mod utilities;
mod xml;

use crate::utilities::{change_counters, get_accept_header};
use crate::xml::Xml;
use actix_web::body::EitherBody;
use actix_web::web::{Data, Json, Query};
use actix_web::{delete, get, main, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::ops::Deref;

use std::ptr::null;

use actix_web::error::ContentTypeError::{ParseError, UnknownEncoding};
use actix_web::error::ReadlinesError::ContentTypeError;
use actix_web::http::StatusCode;
use anyhow::Error;
use mime::Mime;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use wasm_bindgen::JsValue;
use wasmtime::{Caller, Engine, Extern, Func, Linker, Memory, Module, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};

#[derive(Default, Serialize, Debug)]
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

impl ToString for CustomCounterQuery {
    fn to_string(&self) -> String {
        let mut res = String::new();
        res.push_str(self.n.as_str());
        res.push_str(self.a.as_str());
        res.push_str(self.m.as_str());
        res.push_str(self.e.as_str());
        // if let Some(val) = self.counter {
        //     res.push_str(val.to_string().as_str());
        // }
        return res;
    }
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

#[derive(Serialize, Deserialize)]
struct CalcParams {
    first: f64,
    second: f64,
    operation: char,
}

fn print_memory(store: &Store<WasiCtx>, memory: &Memory, ptr: i32, len: i32) {
    println!("Memory pointer in print memory: {}", ptr as u32 as usize);
    let data = memory
        .data(store)
        .get(ptr as u32 as usize..)
        .and_then(|arr| arr.get(..len as u32 as usize));

    println!("Data from memory: {:?}", data);
    let string = match data {
        Some(data) => match std::str::from_utf8(data) {
            Err(err) => { println!("Parsing string error: {}", err.to_string()); return; },
            Ok(s) => s,
        },
        None => { println!("Data is empty!"); return; },
    };
    println!("Memory check: {}", string);
}

#[get("/")]
async fn index(counters: Data<Counters>, req: HttpRequest) -> impl Responder {
    let query = req.query_string();
    let mutex = counters.custom_counters.clone();
    let mut custom_counters_map = match serde_json::to_string(&counters) {
        Ok(data) => data,
        Err(_) => String::new(),
    };
    if let Ok(val) = serde_json::to_string(&*mutex.lock().await) {
        custom_counters_map.pop();
        custom_counters_map.push_str(format!(", \"custom_counters\":{}", val).as_str());
        custom_counters_map.push('}');
    }
    let mem_size = (query.len() + custom_counters_map.len()) * 2;
    println!("Memory size: {mem_size}");

    let files = match fs::read_dir("./") {
        Ok(val) => val,
        Err(err) => {
            return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(err.to_string())
        }
    };

    let mut modules = vec![];

    for file in files.flatten() {
        if let Some(file_name) = file.file_name().clone().to_str() {
            if file_name.contains(".wasm") {
                modules.push(file_name.to_string());
            }
        }
    }

    let engine = Engine::default();
    for module_name in modules {
        let mut linker = Linker::new(&engine);

        let wasi = match WasiCtxBuilder::new().inherit_stdio().inherit_args() {
            Ok(builder) => builder.build(),
            Err(err) => {
                return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(err.to_string())
            }
        };

        let mut store = Store::new(&engine, wasi);

        if let Err(err) = wasmtime_wasi::add_to_linker(&mut linker, |s| s) {
            return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(err.to_string());
        }
        let module = match Module::from_file(&engine, module_name) {
            Ok(m) => m,
            Err(err) => {
                return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(err.to_string())
            }
        };
        // linker.func_wrap("env", "get_result", get_result).unwrap();
        let instance = match linker.instantiate(&mut store, &module) {
            Ok(inst) => inst,
            Err(err) => {
                return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(err.to_string())
            }
        };
        let memory = match instance.get_memory(&mut store, "memory") {
            Some(mem) => mem,
            None => {
                return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body("Memory error!")
            }
        };

        if let Ok(alloc) = instance.get_typed_func::<i32, i32>(&mut store, "alloc") {
            let ptr = match alloc.call(&mut store, mem_size as i32) {
                Ok(offset) => offset,
                Err(err) => {
                    return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(err.to_string())
                }
            };
            if let Err(err) = memory.write(&mut store, ptr as usize, query.as_bytes()) {
                return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(err.to_string());
            }
            if let Err(err) = memory.write(
                &mut store,
                ptr as usize + query.len() + 1,
                custom_counters_map.as_bytes(),
            ) {
                return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(err.to_string());
            }

            print_memory(&store, &memory, ptr, mem_size as i32);

            let load_data = instance
                .get_typed_func::<(), i32>(&mut store, "load_data_to_wasm")
                .unwrap();
            let new_ptr = load_data.call(&mut store, ()).unwrap();
            println!("After loading data");

            let mut  vec= vec![0; mem_size - 20];

            print_memory(&store, &memory, new_ptr, mem_size as i32 - 20);

            memory.read(&mut store, new_ptr as usize, vec.as_mut_slice()).unwrap();
            println!("Response data: {vec:?}");
            let response_data = String::from_utf8(vec).unwrap();
            let mut response = String::new();
            for res_iter in response_data.chars() {
                if res_iter == '\0' {
                    break;
                }
                response.push(res_iter as char);
            }
            println!("{response}");

        }

        if let Ok(main) = instance.get_typed_func::<(i32, i32), i32>(&mut store, "main") {
            if let Err(err) = main.call(&mut store, (0, 0)) {
                return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(err.to_string());
            }
        }
    }

    HttpResponse::build(StatusCode::OK).body("Completed")
}

#[main]
async fn main() -> std::io::Result<()> {
    let counters: Data<Counters> = Data::new(Counters::default());

    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&counters))
            .service(counter_get)
            .service(counter_delete)
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
