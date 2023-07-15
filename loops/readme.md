# Loops in Rust

Welcome to the "Loops in Rust" section of this repository. This section covers the concept of looping in Rust and provides an overview of the different loop constructs available.

## Loop Types

### `loop` Loop

The `loop` loop is a basic loop construct that executes a block of code indefinitely until an explicit `break` statement is encountered. It is useful when you want to create an infinite loop or loop until a specific condition is met.

### `while` Loop

The `while` loop repeatedly executes a block of code as long as a specified condition is true. It is useful when you know the condition before entering the loop.

```rust
fn main() {
    let mut count = 0;

    while count < 5 {
        println!("Count: {}", count);
        count += 1;
    }
}
```

### `for` Loop

The `for` loop is used to iterate over a sequence of values, such as arrays, ranges, or collections. It is useful when you want to perform a specific operation for each element in the sequence.

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5];

    for number in numbers.iter() {
        println!("Number: {}", number);
    }
}
```

## Loop Control Flow

In addition to the basic loop constructs, Rust provides control flow statements that allow you to control the flow within a loop. These statements include:

- `break`: Terminates the loop and moves to the next statement after the loop.
- `continue`: Skips the rest of the current iteration and moves to the next iteration.
- `return`: Exits the loop and returns a value from the function.

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5];

    for number in numbers.iter() {
        if *number == 3 {
            continue; // Skip the rest of the iteration
        }

        if *number == 5 {
            break; // Exit the loop
        }

        println!("Number: {}", number);
    }

    println!("Loop finished");
}
```

## Examples

This section provides examples to illustrate the usage of loops in Rust. It includes code snippets demonstrating how to use each type of loop and showcases common scenarios where loops are typically used.

Feel free to explore the examples and experiment with loops in Rust. Modify the code, run it, and observe the output to gain a deeper understanding of how loops work in different scenarios.

## Conclusion

Loops are an essential programming construct that allows you to repeat a block of code until a specific condition is met or to iterate over a sequence of values. Understanding how to use loops effectively is fundamental for writing efficient and concise Rust programs.

Take your time to grasp the concepts presented in this section and practice implementing loops in your own code. By mastering loops, you will be able to build dynamic and iterative programs that can handle a wide range of scenarios.
