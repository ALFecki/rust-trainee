use std::{collections::HashMap, io};

fn main() {
    println!("Enter your string, please: ");

    let mut user_string = String::new();

    io::stdin()
        .read_line(&mut user_string)
        .expect("Failed to read line");

    let table = symbol_count(&user_string);
    println!("Symbols in your string are: {:?}", table);
    loop {
        println!("Enter symbol, which you want to check or enter empty string to close the app: ");
        user_string.clear();
        match io::stdin().read_line(&mut user_string) {
            Ok(n) => {
                if user_string == "\r\n" {
                    break;
                }
                match unique_symbol(&user_string, &table) {
                    true => println!("Symbol is unique"),
                    false => println!("Symbol isn't unique or doesn't exist"),
                }
            }
            Err(_) => println!("Error reading string"),
        }
    }
}

fn unique_symbol(str: &String, table: &HashMap<char, i32>) -> bool {
    if str.chars().count() > 3 {
        println!("String is too long, we need 1 symbol");
        return false;
    }
    let symbol = str.chars().next().unwrap();
    if table.contains_key(&symbol) && table.get_key_value(&symbol) == Some((&symbol, &1)) {
        return true;
    }
    return false;
}

fn symbol_count(str: &String) -> HashMap<char, i32> {
    let mut table: HashMap<char, i32> = HashMap::new();
    let chars = str.chars();
    for symbol in chars {
        table
            .entry(symbol)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    table
}
