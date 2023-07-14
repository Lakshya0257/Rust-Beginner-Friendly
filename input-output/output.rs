fn main() {
    // Print to the console using println!
    println!("Hello, Rust!");

    // Print formatted output using print!
    let name = "Alice";
    let age = 25;
    print!("My name is {} and I am {} years old.", name, age);

    // String interpolation using format!
    let x = 10;
    let y = 20;
    let sum = x + y;
    let product = x * y;
    let difference = x - y;
    let quotient = x / y;
    let modulus = x % y;

    println!("The sum is {}, product is {}, difference is {}, quotient is {}, modulus is {}",
             sum, product, difference, quotient, modulus);

    // Writing to a string using format!
    let message = format!("My name is {} and I am {} years old.", name, age);
    println!("{}", message);

    // Standard error output
    eprintln!("This is an error message.");

    // Returning a value from a function
    let result = multiply(5, 7);
    println!("The result of multiplication is: {}", result);
}

// A simple multiplication function
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
