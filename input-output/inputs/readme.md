# Input in Rust

This code example demonstrates how to take user input in Rust for different data types.

## Integer Input
To take an integer input, we follow these steps:
1. Create a mutable string variable using `let mut variable_name = String::new();`.
2. Print a prompt message using `println!("Enter an integer:");`.
3. Read the user input into the string variable using `io::stdin().read_line(&mut variable_name).expect("Failed to read input");`.
4. Parse the input string as an `i32` using `let variable_name: i32 = variable_name.trim().parse().expect("Invalid input");`.

## Floating-Point Input
To take a floating-point input, we follow similar steps as integer input, but with the appropriate data type. For example:
1. Create a mutable string variable: `let mut variable_name = String::new();`.
2. Print a prompt message.
3. Read the user input.
4. Parse the input string as an `f64`.

## String Input
To take a string input, we follow these steps:
1. Create a mutable string variable: `let mut variable_name = String::new();`.
2. Print a prompt message.
3. Read the user input.
4. Trim any whitespace from the input using `variable_name.trim()`.
5. Use the resulting trimmed string as the input.

## Character Input
To take a character input, we follow similar steps as string input, but with a few adjustments. For example:
1. Create a mutable string variable: `let mut variable_name = String::new();`.
2. Print a prompt message.
3. Read the user input.
4. Retrieve the first character from the input using `char_input.chars().next().expect("Invalid input")`.

## Boolean Input
To take a boolean input, we follow similar steps as string input, but parse the input string as a `bool`. For example:
1. Create a mutable string variable: `let mut variable_name = String::new();`.
2. Print a prompt message.
3. Read the user input.
4. Parse the input string as a `bool`.

Feel free to modify this code example to suit your specific requirements and add error handling or validation logic as needed.

