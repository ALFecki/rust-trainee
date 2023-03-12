use std::collections::HashMap;
use std::ffi::{c_void, CStr};
use std::mem;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use serde::{Serialize, Deserialize};


extern "C" {
    fn log_str(ptr: i32, len: i32);
    fn set_str(ptr: i32);
}

#[derive(Default, Serialize)]
pub struct Counters {
    counter: AtomicU32,
    delete_counter: AtomicU32,
    // #[serde(skip_serializing)]
    // custom_counters: Arc<Mutex<HashMap<String, AtomicU32>>>,
}

#[derive(Deserialize, Serialize, Debug)]
struct CustomCounterQuery {
    n: String,
    a: String,
    m: String,
    e: String,
    counter: Option<u32>
}

impl ToString for CustomCounterQuery {
    fn to_string(&self) -> String {
        let mut res = String::new();
        res.push_str(self.n.as_str());
        res.push_str(self.a.as_str());
        res.push_str(self.m.as_str());
        res.push_str(self.e.as_str());
        if let Some(val) = self.counter {
            res.push_str(val.to_string().as_str());
        }
        return res;
    }
}

#[derive(Deserialize)]
struct CustomAdd {
    counter: u32,
}


#[no_mangle]
pub unsafe extern "C" fn alloc() {
    let mut buf = Vec::with_capacity(1024);
    let ptr: *mut c_void = buf.as_mut_ptr();
    // log_str(ptr as i32, 1024);
    // mem::forget(buf);
}

pub unsafe extern "C"  fn dealloc(ptr: *mut u8) {

    let data = Vec::from_raw_parts(ptr , 0, 1024);
    std::mem::drop(data);
}

#[no_mangle]
pub unsafe extern "C" fn change_counters(ptr: *mut u8) {
    let query = CStr::from_ptr(ptr as *const i8).to_str().unwrap();
    println!("{}", query);
}





fn main() {
    // let a = alloc();
    // unsafe { dealloc(a); }

}
