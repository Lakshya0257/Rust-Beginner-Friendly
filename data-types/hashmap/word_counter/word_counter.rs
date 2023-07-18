use std::collections::HashMap;
use std::io;

fn main() {
    let mut line: String = String::new();

    println!("Input a line");
    io::stdin().read_line(&mut line).expect("Invalid");

    let mut map: HashMap<String, i32> = HashMap::new();

    for word in line.split_whitespace() {
        let count = map.entry(word.to_string()).or_insert(0);
        *count += 1;
    }

    for (key, value) in map {
        println!("Key: {}", key);
        println!("Value: {}", value);
    }
}
