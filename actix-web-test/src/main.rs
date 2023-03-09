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
use std::fs::FileType;

use actix_web::error::ContentTypeError::{ParseError, UnknownEncoding};
use actix_web::error::ReadlinesError::ContentTypeError;
use mime::Mime;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use wasmer::{Engine, FunctionEnv, imports, Instance, MemoryView, Module, Store, TypedFunction, WasmPtr, WasmRef};
use wasmer_wasi::{generate_import_object_from_env, get_wasi_version, WasiState};

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


#[get("/")]
async fn index(counters: Data<Counters>,
               params: Option<Query<CustomCounterQuery>>,
               counter: Option<Query<CustomAdd>>
) -> impl Responder {
    let mut response = String::new();

    let files = match  fs::read_dir("./")  {
        Ok(val) => val,
        Err(err) => return err.to_string()
    };

    let mut modules = vec![];

    for file in files {
        if let Ok(f) = file {
            if let Some(file_name) = f.file_name().clone().to_str() {
                if file_name.contains(".wasm") {
                    modules.push(file_name.to_string());
                }
            }
        }
    }

    for module in modules {

        let mut store = Store::default();
        let wasm_bytes = fs::read(module).unwrap();
        let module = Module::new(&store, wasm_bytes).unwrap();
        for export in module.exports() {
            println!("{:?}", export);
        }

        let wasi_env = WasiState::new("command-name").finalize(&mut store).unwrap();

        let version = get_wasi_version(&module, true).unwrap();
        let imports = generate_import_object_from_env(&mut store, &wasi_env.env, version);
        let instance = Instance::new(&mut store, &module, &imports).unwrap();

        let mem = instance.context_mut().memory(0).allocate(8);




        // let module = Module::from_file(&engine, module.as_str()).unwrap();
        //
        //
        // let mut linker = Linker::new(&engine);
        // let wasi = WasiCtxBuilder::new()
        //     .inherit_stdio()
        //     .inherit_args().unwrap()
        //     .build();
        // let mut store = Store::new(&engine, wasi);
        // wasmtime_wasi::add_to_linker(&mut linker, |s| s).unwrap();
        // let link = linker.instantiate(&mut store, &module).unwrap();
        // let memory = link.get_memory(&mut store, "memory").unwrap();
        //
        // let buf = memory.data_ptr(&mut store);
        //
        // memory.write(&mut store, 256, b"389weyfw").unwrap();
        // let change = link.get_typed_func::<*mut u8, ()>(&mut store, "change_counters").unwrap();
        // change.call(&mut store, buf).unwrap();




        // let alloc = instance.get_typed_func::<(), &u8>(&mut store, "alloc").unwrap();
        // let alloc = instance.get_func(&mut store, "alloc").ok_or(Err("Something"))?.get2::<i32, i32>()?;
        // let alloc = link.get_typed_func::<(), ()>(&mut store, "alloc").unwrap();
        // alloc.call(&mut store, ()).unwrap();


        // let buf = alloc.call(&mut store, &[], &mut []).unwrap();




        // for export in module.exports() {
        //     println!("{:?}", export);
        // }
        // let memory = link.get_memory(&store, "memory").unwrap();
        // let alloc_fn = link.get_func(&store, "alloc").unwrap();
        // let alloc_fn = link.get_typed_func::<(), >(&mut store, "alloc").unwrap();
        // let buf = alloc_fn.call(store, ()).unwrap();
        // memory.write(&store, memory.data_size(&store), &buf).unwrap();
        // let add_fn = link.get_typed_func::<*mut, String>(&mut store, "change_counters").unwrap();
        // response.push_str(format!("Counter: {}", add_fn.call(store, ().unwrap()).as_str());
        // println!("{:?}", add_fn.call(store, (1, 1, 2)).unwrap());
    }

    // for module in modules {
    //
    // }
    return response;
    // return Err(());

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
