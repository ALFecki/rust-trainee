use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

extern "C" {
    fn get_input_size() -> i32;
    fn set_input(ptr: i32);
    fn get_output(ptr: i32, len: i32);
}

#[derive(Default, Debug, Serialize, Deserialize)]
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
pub extern "C" fn alloc(size: i32) -> *mut u8 {
    let mut buf = Vec::with_capacity(size as usize);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn dealloc(ptr: *mut u8, size: i32) {
    let data = Vec::from_raw_parts(ptr, size as usize, size as usize);
    std::mem::drop(data);
}

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
        let map = &mut counters.custom_counters;

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

fn main() -> anyhow::Result<()> {
    let mem_size = unsafe { get_input_size() };
    let ptr = alloc(mem_size);

    let input_buf = unsafe {
        set_input(ptr as i32);
        String::from_raw_parts(ptr, mem_size as usize, mem_size as usize)
    };
    println!("Input buf: {}", input_buf);

    let input_buf = input_buf.split('\0').collect::<Vec<&str>>();
    let edited = match serde_json::from_str::<Counters>(input_buf[1]) {
        Ok(counters) => custom_counter(
            counters,
            serde_qs::from_str::<CustomCounterQuery>(input_buf[0]).ok(),
            serde_qs::from_str::<CustomAdd>(input_buf[0]).ok(),
        ),
        Err(err) => anyhow::bail!("Error deserializing input in wasm: {}", err.to_string()),
    };
    let output = match serde_json::to_vec(&edited) {
        Ok(out) => out,
        Err(err) => anyhow::bail!("Error serializing output: {}", err.to_string()),
    };
    unsafe { get_output(output.as_ptr() as i32, output.len() as i32); }
    println!("{:?}", output);
    unsafe { dealloc(ptr, mem_size) };
    Ok(())
}
