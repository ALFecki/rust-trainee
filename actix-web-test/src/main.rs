mod utilities;
mod xml;

use crate::utilities::{change_counters, get_accept_header};
use crate::xml::Xml;
use actix_web::body::EitherBody;
use actix_web::error::ContentTypeError::{ParseError, UnknownEncoding};
use actix_web::error::ReadlinesError::ContentTypeError;
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json, Query};
use actix_web::{delete, get, main, App, HttpRequest, HttpResponse, HttpServer, Responder};
use mime::Mime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::ops::Deref;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use wasmtime::{Caller, Engine, Extern, Linker, Memory, MemoryType, Module, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};

#[derive(Default, Serialize, Deserialize, Debug)]
struct Counters {
    counter: AtomicU32,
    delete_counter: AtomicU32,
    #[serde(skip_serializing, skip_deserializing)]
    custom_counters: Arc<Mutex<HashMap<String, AtomicU32>>>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
struct CountersDto {
    counter: u32,
    delete_counter: u32,
    custom_counters: HashMap<String, u32>,
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

#[derive(Serialize, Deserialize)]
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

#[get("/")]
async fn new_index(counters: Data<Counters>, calc_params: Option<Json<CalcParams>>, req: HttpRequest) -> impl Responder {
    let custom_counters = counters.custom_counters.clone();
    let mut mutex = custom_counters.lock().await;

    let query = req.query_string().to_string();
    let mut map_copy = HashMap::new();
    for (k, v) in &*mutex {
        map_copy.insert((*k).clone(), v.load(Ordering::SeqCst));
    }

    let request = match serde_json::to_string(&CountersDto {
        counter: counters.counter.load(Ordering::SeqCst),
        delete_counter: counters.delete_counter.load(Ordering::SeqCst),
        custom_counters: map_copy,
    }) {
        Ok(ser_res) => {
            let mut result = String::new();
            if !query.is_empty() {
                result.push_str(format!("{query}\0").as_str());
            }
            result += &ser_res;
            if let Some(calc) = calc_params {
                if let Ok(calc) = serde_json::to_string(calc.deref()) {
                    result.push_str(format!("\0{}", calc).as_str())
                }
            }
            result
        },
        Err(err) => {
            return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(err.to_string())
        }
    };
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
        let input = request.as_bytes().to_vec();
        let input_size = input.len() as i32;

        let wasi = match WasiCtxBuilder::new().inherit_stdio().inherit_args() {
            Ok(builder) => builder.build(),
            Err(err) => {
                return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(err.to_string())
            }
        };

        let mut store = Store::new(&engine, wasi);
        let memory_type = MemoryType::new(1, None);
        if let Err(err) = Memory::new(&mut store, memory_type) {
            return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(err.to_string());
        }

        if let Err(err) = linker.func_wrap("env", "get_input_size", move || -> i32 { input_size }) {
            return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(err.to_string());
        }

        if let Err(err) = linker.func_wrap(
            "env",
            "set_input",
            move |mut caller: Caller<'_, WasiCtx>, ptr: i32| {
                let mem = match caller.get_export("memory") {
                    Some(Extern::Memory(mem)) => mem,
                    _ => anyhow::bail!("Failed to find memory"),
                };
                let offset = ptr as u32 as usize;
                match mem.write(&mut caller, offset, &input) {
                    Ok(_) => {}
                    _ => anyhow::bail!("Failed to write memory"),
                };
                Ok(())
            },
        ) {
            return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(err.to_string());
        }

        let buffer_for_result: Arc<parking_lot::Mutex<Vec<u8>>> =
            Arc::new(parking_lot::Mutex::new(vec![]));
        let buf_clone = buffer_for_result.clone();
        if let Err(err) = linker.func_wrap(
            "env",
            "get_output",
            move |mut caller: Caller<'_, WasiCtx>, ptr: i32, len: i32| {
                let mem = match caller.get_export("memory") {
                    Some(Extern::Memory(mem)) => mem,
                    _ => anyhow::bail!("Failed to find memory"),
                };
                let offset = ptr as u32 as usize;
                let mut buffer: Vec<u8> = vec![0; len as usize];
                if let Err(err) = mem.read(&mut caller, offset, &mut buffer) {
                    anyhow::bail!("Memory access error: {}", err.to_string())
                }
                let mut buf = buf_clone.lock();
                if let Err(err) = buf.write_all(&buffer) {
                    anyhow::bail!("Error reading output: {}", err.to_string())
                }
                Ok(())
            },
        ) {
            return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(err.to_string());
        }

        if let Err(err) = wasmtime_wasi::add_to_linker(&mut linker, |s| s) {
            return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(err.to_string());
        }
        let module = match Module::from_file(&engine, module_name) {
            Ok(m) => m,
            Err(err) => {
                return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(err.to_string())
            }
        };
        let instance = match linker.instantiate(&mut store, &module) {
            Ok(inst) => inst,
            Err(err) => {
                return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(err.to_string())
            }
        };

        if let Ok(start) = instance.get_typed_func::<(), ()>(&mut store, "_start") {
            if let Err(_) = start.call(&mut store, ()) {
                continue;
            }
        }

        let response = String::from_utf8((*buffer_for_result.lock()).clone());

        println!(
            "Result is {:?}", response
        );
        let result = match response {
            Ok(val) => val,
            Err(err) => {
                return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(err.to_string())
            }
        };
        if let Ok(deserialized_result) = serde_json::from_str::<CountersDto>(result.as_str()) {
            counters
                .counter
                .store(deserialized_result.counter, Ordering::SeqCst);
            counters
                .delete_counter
                .store(deserialized_result.delete_counter, Ordering::SeqCst);
            for (k, v) in deserialized_result.custom_counters {
                mutex.insert(k, AtomicU32::new(v));
            }
        };

        // other modules results
    }
    HttpResponse::build(StatusCode::OK).body("Request done!")
}

#[main]
async fn main() -> std::io::Result<()> {
    let counters: Data<Counters> = Data::new(Counters::default());

    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&counters))
            .service(counter_get)
            .service(counter_delete)
            .service(new_index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
