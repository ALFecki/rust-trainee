use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::atomic::{AtomicU32, Ordering};
use wasm_util::*;

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
    // counter: Option<u32>,
}

#[derive(Deserialize)]
pub struct CustomAdd {
    counter: String,
}

pub extern "C" fn change_counters(incr: &AtomicU32, to_swap: &AtomicU32, to_add: u32) -> u32 {
    to_swap.swap(0, Ordering::SeqCst);
    incr.fetch_add(to_add, Ordering::SeqCst) + to_add
}

pub extern "C" fn custom_counter(
    mut counters: Counters,
    query: &Option<CustomCounterQuery>,
    query_to_add: &Option<CustomAdd>,
) -> Counters {
    let mut to_add = match query_to_add {
        Some(val) => {
            let mut res = 1;
            if let Ok(r) = u32::from_str(val.counter.as_str()) {
                res = r;
            }
            res
        } ,
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
    let (query_custom_counter, query_custom_add) = match input_buf.first() {
        Some(f) => (
            serde_qs::from_str::<CustomCounterQuery>(f).ok(),
            serde_qs::from_str::<CustomAdd>(f).ok(),
        ),
        None => (None, None),
    };
    let mut edited = Counters::default();
    for str in input_buf {
        if let Ok(counters) = serde_json::from_str::<Counters>(str) {
            edited = custom_counter(counters, &query_custom_counter, &query_custom_add);
            break;
        }
    }
    let output = match serde_json::to_vec(&edited) {
        Ok(out) => out,
        Err(err) => anyhow::bail!("Error serializing output: {}", err.to_string()),
    };

    unsafe {
        get_output(output.as_ptr() as i32, output.len() as i32);
    }
    unsafe { dealloc(ptr, mem_size) };
    Ok(())
}
