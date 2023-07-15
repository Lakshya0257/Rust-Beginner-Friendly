use std::io;

fn main() {
    // Integer Input
    let mut integer_input = String::new();
    println!("Enter an integer:");
    io::stdin()
        .read_line(&mut integer_input)
        .expect("Failed to read input");
    let integer_input: i32 = integer_input.trim().parse().expect("Invalid input");

    // Floating-Point Input
    let mut float_input = String::new();
    println!("Enter a floating-point number:");
    io::stdin()
        .read_line(&mut float_input)
        .expect("Failed to read input");
    let float_input: f64 = float_input.trim().parse().expect("Invalid input");

    // String Input
    let mut string_input = String::new();
    println!("Enter a string:");
    io::stdin()
        .read_line(&mut string_input)
        .expect("Failed to read input");
    let string_input = string_input.trim();

    // Character Input
    let mut char_input = String::new();
    println!("Enter a character:");
    io::stdin()
        .read_line(&mut char_input)
        .expect("Failed to read input");
    let char_input: char = char_input.chars().next().expect("Invalid input");

    // Boolean Input
    let mut bool_input = String::new();
    println!("Enter a boolean value (true/false):");
    io::stdin()
        .read_line(&mut bool_input)
        .expect("Failed to read input");
    let bool_input: bool = bool_input.trim().parse().expect("Invalid input");

    // Displaying the inputs
    println!("Integer Input: {}", integer_input);
    println!("Floating-Point Input: {}", float_input);
    println!("String Input: {}", string_input);
    println!("Character Input: {}", char_input);
    println!("Boolean Input: {}", bool_input);
}
