use std::collections::HashMap;
use std::io;
fn main() {
    let mut db = HashMap::new();
    loop {
        let mut input = String::new();
        println!("Enter a command");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim();
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "SET" => {
                if parts[0] == "SET" && parts.len() != 3 {
                    println!("Invalid syntax, SET expects 2 argument(s)");
                    continue;
                }
                db.insert(parts[1].to_string(), parts[2].to_string());
                println!("{:?}", db);
            }
            "GET" => {
                if parts[0] == "GET" && parts.len() != 2 {
                    println!("Invalid syntax, GET expects 1 argument(s)");
                }
                match db.get(parts[1]) {
                    Some(value) => {
                        println!("{}", value);
                    }

                    None => {
                        println!("Key Not Found");
                    }
                }
            }
            _ => {
                println!("UNKNOWN COMMAND");
            }
        }
    }
}
