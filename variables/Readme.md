## Main Function

The `main` function is the entry point of a Rust program. It serves as the starting point of execution. In this code, it showcases the following concepts:

- Printing Messages: The `println!` macro is used to display messages in the console. It is a convenient way to output text during program execution.

- Variable Declaration: The `let` keyword is used to declare variables. In Rust, variables are immutable by default, so they cannot be changed after initialization unless specified as mutable using the `mut` keyword.

- Assertion: The `assert_eq!` macro is used to perform assertions in Rust. It compares two values and terminates the program if they are not equal. Assertions are commonly used for testing and ensuring correctness in programs.

### Mutable Declaration

The `mutable` function demonstrates the concept of mutable variables in Rust. In Rust, variables are immutable by default, meaning they cannot be modified after initialization. However, by using the `mut` keyword, variables can be declared as mutable, allowing their values to be changed.

### Shadowing

The `shadowing` function illustrates the concept of variable shadowing in Rust. Shadowing allows declaring a new variable with the same name as an existing variable, effectively "shadowing" the previous variable. The new variable has a narrower scope and may have a different type or value, allowing more flexibility in code. Shadowing is useful for reusing variable names while maintaining clarity and preventing accidental modification of variables.

### Scope Declaration

Within the `main` function, a scoped block is defined using curly braces `{}`. This block creates a new scope, which is a region of code where variables are valid and accessible. Variables declared within a scope are only visible and usable within that scope. Once the scope ends, the variables declared within it are destroyed, and their memory is freed. Scoping allows for better control of variable lifetimes and prevents naming conflicts between variables in different scopes.

## Conclusion

A detailed explanation of the concepts used in the provided Rust code. It covers the main function, variable mutability, variable shadowing, and scoped variable declaration. Understanding these concepts is crucial for writing idiomatic and effective Rust programs.
