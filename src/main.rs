// use std::collections::HashMap;

// fn main() {
//     let mut db = HashMap::new();
//     db.insert(String::from("name"), String::from("Aaron"));
//     db.insert(String::from("city"),String::from("Pune"));
//     db.insert(String::from("language"),String::from("Rust"));
//     println!("{:?}", db);
// }

// use std::collections::HashMap;
use std::io;
fn main() {
    // let mut db = HashMap::new();
    loop {
        let mut input = String::new();
        println!("Enter a command");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim();
        let parts: Vec<&str> = input.split_whitespace().collect();
        println!("The input is {input}");
        println!("These are the parts: {:?}",parts)
    }
}
