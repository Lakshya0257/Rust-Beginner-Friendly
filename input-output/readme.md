# Input and Output in Rust

## `output.rs`

This section describes different ways to output data in Rust.

- **Print to the Console**: Use the `println!` macro to print data to the console with a newline at the end. It accepts format specifiers for displaying values.

- **Formatted Output**: The `print!` macro allows formatted output without a newline. It works similarly to `println!` but does not append a newline character.

- **String Interpolation**: Rust provides the `format!` macro to perform string interpolation. It allows you to include values within a string by specifying placeholders.

- **Writing to a File**: Rust allows writing data to a file using the `writeln!` macro. You need to import the necessary modules and create a file object. Then, use the macro to write data to the file.

- **Standard Error Output**: Rust provides the `eprintln!` macro to print error messages to the standard error stream. It is useful for printing error information or diagnostic messages.

- **Returning a Value from a Function**: Rust functions can return values. You can define functions that perform calculations or operations and return a result using the `return` keyword.

Feel free to explore these different techniques for outputting data in Rust based on your specific requirements.

