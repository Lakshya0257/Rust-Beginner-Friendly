# Rust Data Type Declarations

This README file provides an overview of various data type declarations in Rust programming language. Rust is a statically typed language, which means that every variable must have a specific type declared at compile-time.

## Primitive Data Types

Rust has several primitive data types, each with its own characteristics and memory requirements. The following table summarizes the commonly used primitive data types in Rust:

| Data Type | Description                                   | Size (in bytes) | Range                             |
|-----------|-----------------------------------------------|-----------------|-----------------------------------|
| `bool`    | Represents a boolean value (`true` or `false`) | 1               | `true` or `false`                  |
| `i8`      | Signed 8-bit integer                          | 1               | -128 to 127                       |
| `i16`     | Signed 16-bit integer                         | 2               | -32,768 to 32,767                 |
| `i32`     | Signed 32-bit integer                         | 4               | -2,147,483,648 to 2,147,483,647   |
| `i64`     | Signed 64-bit integer                         | 8               | -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807 |
| `u8`      | Unsigned 8-bit integer                        | 1               | 0 to 255                          |
| `u16`     | Unsigned 16-bit integer                       | 2               | 0 to 65,535                       |
| `u32`     | Unsigned 32-bit integer                       | 4               | 0 to 4,294,967,295                |
| `u64`     | Unsigned 64-bit integer                       | 8               | 0 to 18,446,744,073,709,551,615    |
| `f32`     | 32-bit floating-point number                   | 4               | ±1.18E-38 to ±3.4E+38             |
| `f64`     | 64-bit floating-point number                   | 8               | ±2.23E-308 to ±1.8E+308           |
| `char`    | Unicode scalar value                           | 4               | 0 to 1,114,111 (inclusive)         |
| `String`  | UTF-8 encoded string                           | Variable        | Variable                          |

## Detailed Explanation

### `bool`

The `bool` type represents a boolean value, which can be either `true` or `false`. It is the simplest data type in Rust, occupying 1 byte of memory.

### Signed Integers

Rust provides signed integers of various sizes, such as `i8`, `i16`, `i32`, and `i64`. The `i` prefix indicates that these are signed integers. For example, `i32` represents a signed 32-bit integer, which can hold values ranging from -2,147,483,648 to 2,147,483,647. The number following `i` represents the number of bits used to store the integer, determining its range and memory usage.

### Unsigned Integers

Similarly, Rust provides unsigned integers of various sizes, such as `u8`, `u16`, `u32`, and `u64`. The `u` prefix indicates that these are unsigned integers. For example, `u32` represents an unsigned 32-bit integer, which can hold values ranging from 0 to 4,294,967,295. Unsigned integers use the same bit sizes as their signed counterparts.

### Floating-Point Numbers

Rust supports floating-point numbers of two sizes: `f32` and `f64`. The `f` prefix indicates a floating-point number. For example, `f32` represents a 32-bit floating-point number, which can store decimal values with a precision of approximately 6 decimal places. On the other hand, `f64` represents a 64-bit floating-point number, providing a higher precision of approximately 15 decimal places.

### `char`

The `char` type represents a single Unicode scalar value, such as a letter, digit, or symbol. In Rust, `char` takes 4 bytes of memory and can represent any Unicode character from the Basic Multilingual Plane (BMP), which includes most commonly used characters.

### `String`

The `String` type represents a dynamically allocated, mutable UTF-8 encoded string. Unlike string literals (`&str`), which are immutable and stored in the program's binary, `String` objects can be modified and are stored on the heap. The size of a `String` can vary depending on the length of the string it holds.

## Conclusion

Understanding the different data types available in Rust is crucial for writing efficient and bug-free programs. This README file provides a brief overview of the commonly used data types, including the `String` type for handling strings. For more detailed information, consult the Rust documentation.
