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
        if  parts[0] == "SET" && parts.len() != 3 {
            println!("Please enter exactly 3 words");
            continue;
        }
        match parts[0] {
            "SET" => {
                db.insert(parts[1].to_string(), parts[2].to_string());
                println!("{:?}", db);
            }
            "GET" => {
                println!("{:?}",db.get(parts[1]));
            }
            _ => {
                println!("UNKNOWN COMMAND");
            }
        }
    }
}
