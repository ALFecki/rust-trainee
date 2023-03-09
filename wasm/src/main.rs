use std::collections::HashMap;
use std::ffi::{c_void, CStr};
use std::mem;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use serde::{Serialize, Deserialize};

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
pub extern "C" fn alloc() -> *mut c_void {
    let mut buf = Vec::with_capacity(1024);
    let ptr = buf.as_mut_ptr();

    mem::forget(buf);

    ptr

}

pub unsafe extern "C"  fn dealloc(ptr: *mut u8) {
    let _ = Vec::from_raw_parts(ptr, 0, 1024);
}

#[no_mangle]
pub unsafe extern "C" fn change_counters(ptr: *mut u8) {
    let query = CStr::from_ptr(ptr as *const i8).to_str().unwrap();
    println!("{}", query);
    // if let Some(custom_add) =

    // to_swap.swap(0, Ordering::SeqCst);
    // incr.fetch_add(to_add, Ordering::SeqCst) + to_add
}





fn main() {
    let a = alloc();
    unsafe { dealloc(a); }

}
