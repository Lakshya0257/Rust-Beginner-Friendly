fn main() {
    let num1: i8 = 14;
    let sum: i8 = add_value(Some(num1), Some(14));
    println!("{}",sum);
}

fn add_value(a: Option<i8>, b: Option<i8>)-> i8 {
    a.unwrap_or(0)+b.unwrap_or(0)
}

