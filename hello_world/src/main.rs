use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Enter the number: ");
        let mut user_string = String::new();

        io::stdin().read_line(&mut user_string).expect("Error!");

        let user_string: u32 = match user_string.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };
        

        match user_string.cmp(&secret_number) {
            Ordering::Equal => { 
                println!("You win!");
                break;
            },

            Ordering::Greater => println!("Your number is greater than secret"),
            Ordering::Less => println!("Your number is less than secret"),
        }
    }
}
