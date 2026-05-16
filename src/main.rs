use std::collections::HashMap;

fn main() {
    let mut db = HashMap::new();
    db.insert(String::from("name"), String::from("Aaron"));
    db.insert(String::from("city"),String::from("Pune"));
    db.insert(String::from("language"),String::from("Rust"));
    println!("{:?}", db);
}
