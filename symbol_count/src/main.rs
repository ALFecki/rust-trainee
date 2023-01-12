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
                if n < 3 {
                    break;
                }
                if n > 3 {
                    println!("String is too long, we need 1 symbol");
                } else {
                    let symbol = user_string.chars().next().unwrap();
                    let unique = unique_symbol(&symbol, &table);
                    if unique {
                        println!("Symbol {symbol} is unique");
                    } else {
                        println!("Symbol {symbol} is not unique or doesn't exist");
                    }
                }
            }
            Err(_) => println!("Error reading string"),
        }
    }
}

fn unique_symbol(symbol: &char, table: &HashMap<char, i32>) -> bool {
    if table.contains_key(symbol) {
        if table.get_key_value(symbol) == Some((&symbol, &1)) {
            return true;
        }
    }
    return false;
}

fn symbol_count(str: &String) -> HashMap<char, i32> {
    let mut table: HashMap<char, i32> = HashMap::new();
    let chars = str.chars();
    for symbol in chars {
        if symbol == '\n' || symbol == '\r' {
            continue;
        }
        table
            .entry(symbol)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    table
}
