use serde::{Deserialize, Serialize};
use wasm_util::*;

#[derive(Serialize, Deserialize, Debug)]
struct Calculator {
    first: f64,
    second: f64,
    operation: char,
}

impl Calculator {
    fn get_result(&self) -> anyhow::Result<f64> {
        Ok(match self.operation {
            '+' => self.first + self.second,
            '-' => self.first - self.second,
            '*' => self.first * self.second,
            '/' => self.first / self.second,
            _ => anyhow::bail!("Invalid operation!"),
        })
    }
}

fn main() -> anyhow::Result<()> {
    let mem_size = unsafe { get_input_size() };
    let ptr = alloc(mem_size);

    let input_buf = unsafe {
        set_input(ptr as i32);
        String::from_raw_parts(ptr, mem_size as usize, mem_size as usize)
    };
    let input_buf = input_buf.split('\0').collect::<Vec<&str>>();
    let input = match input_buf.last() {
        Some(val) => serde_json::from_str::<Calculator>(val).ok(),
        None => None
    };


    let input = match input {
        Some(val) => val,
        None => anyhow::bail!("Input data in calculator is invalid")
    };

    println!("{:?}", input);


    let response = match input.get_result() {
        Ok(res) => format!("result={}", res).as_bytes().to_vec(),
        Err(err) => anyhow::bail!(err.to_string()),
    };

    unsafe {
        get_output(response.as_ptr() as i32, response.len() as i32);
        dealloc(ptr, mem_size)
    }

    Ok(())
}
