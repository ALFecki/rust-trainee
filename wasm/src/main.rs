use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::ffi::{c_void, CStr};
use std::mem;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use serde_with::{serde_as, DisplayFromStr};

static mut MEMORY_POINTER: *mut u8 = null_mut();
static mut MEMORY_SIZE: usize = 0;

extern "C" {
    fn log_str(ptr: i32, len: i32);
    fn set_str(ptr: i32);
    fn deallocate(size: i32);
}

#[derive(Default, Serialize, Deserialize)]
pub struct Counters {
    counter: AtomicU32,
    delete_counter: AtomicU32,
    custom_counters: HashMap<String, AtomicU32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CustomCounterQuery {
    n: String,
    a: String,
    m: String,
    e: String,
    counter: Option<u32>,
}

#[derive(Deserialize)]
pub struct CustomAdd {
    counter: u32,
}

#[no_mangle]
pub unsafe extern "C" fn alloc(size: i32) -> *mut u8 {
    let mut buf = Vec::with_capacity(size as usize);
    let ptr = buf.as_mut_ptr();
    MEMORY_POINTER = ptr;
    MEMORY_SIZE = size as usize;
    std::mem::forget(buf);
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn dealloc() {
    let data = Vec::from_raw_parts(MEMORY_POINTER, MEMORY_SIZE, MEMORY_SIZE);
    std::mem::drop(data);
}

// #[no_mangle]
// pub unsafe extern "C" fn change_counters(ptr: *mut u8) {
//     let query = CStr::from_ptr(ptr as *const i8).to_str().unwrap();
//     println!("{}", query);
// }

pub extern "C" fn change_counters(incr: &AtomicU32, to_swap: &AtomicU32, to_add: u32) -> u32 {
    to_swap.swap(0, Ordering::SeqCst);
    incr.fetch_add(to_add, Ordering::SeqCst) + to_add
}

pub extern "C" fn custom_counter(
    mut counters: Counters,
    query: Option<CustomCounterQuery>,
    query_to_add: Option<CustomAdd>,
) -> Counters {
    let mut to_add = match query_to_add {
        Some(val) => val.counter,
        None => 1,
    };
    if let Some(params) = query {
        let mut name = String::new();
        let mut map = &mut counters.custom_counters;

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
    counters
}

fn main() {
    unsafe {
        let req =  String::from_raw_parts(MEMORY_POINTER, MEMORY_SIZE, MEMORY_SIZE);
        let query = req
            .chars()
            .take_while(|c| *c != '\0')
            .collect::<String>();
        // println!("TEST!!!!!!!!!!! {}", String::from_raw_parts(MEMORY_POINTER.add(query.len() + 1), MEMORY_SIZE, MEMORY_SIZE));
        let custom_counters =
            req.get_unchecked(query.len() + 1..)
                .chars()
                .take_while(|c| *c != '\0')
                .collect::<String>();

        println!("Query: {query}, Custom_counters: {custom_counters}");

        custom_counter(
            serde_qs::from_str::<Counters>(&custom_counters).unwrap(),
            serde_qs::from_str::<CustomCounterQuery>(&query).ok(),
            serde_qs::from_str::<CustomAdd>(&query).ok(),
        );
    }

    unsafe {
        dealloc();
    }
    println!("Deallocated");
    // log_str(memory as i32, size);
}
