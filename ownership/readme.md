# Ownership in Rust

## Introduction

Rust's ownership system is a unique feature designed to ensure memory safety and eliminate common programming errors like null pointer dereferences, dangling pointers, and data races. The ownership concept revolves around three key principles: ownership, borrowing, and lifetimes.

## Ownership

In Rust, every value has an owner, and there can only be one owner at a time. The owner is responsible for cleaning up the value when it goes out of scope.

Before understanding ownership, lets take a look at the memory allocation:

### Memory Allocation

In Rust, memory is allocated only in two different forms- Stack and Heap. Stack memory allocation is mainly used for variables that are of fixed size and limited lifetime. Whereas variables that are dynamic in size are stored in heap.

When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.

```rust
    {
        let s = "hello"; // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
```
s will automatically be deleted when its scope is over, deleting variables that are no longer needed.


Ownership follows a set of rules:

1. **Move Semantics**: When a value is assigned to another variable or passed as a function argument, the ownership of the value is transferred to the new owner, and the previous owner no longer has access to it. This prevents multiple variables from accessing and modifying the same value simultaneously.

2. **Ownership and Function Calls**: When passing ownership to a function, the value is moved to the function's parameter. After the function call, the ownership is returned to the caller, or the value may be moved into another variable within the function.

3. **Returning Ownership**: Functions can return ownership of values to the caller, allowing the caller to become the new owner of the returned value.

### Example

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```
The above function will throw an error as the new owner of Hello is now s2.
Explanation- As we know, s1 has a dynamic size. So it will be stored in the heap memory. While assigning its value to the new variable, instead of creating copy of the previous heap we allocate new owner of that heap making s1, which results in better memory allocation/usage.

Now lets take an example of stack:

``` rust
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
```
The above code will execute successfully without errors as both of the variables are of fixed size. So they are allocated in stack which makes copy of value when user re-assign to the new variable.

## Borrowing

While ownership provides safety and prevents data races, Rust also supports borrowing. Borrowing allows temporary access to a value without taking ownership. Borrowing follows these rules:

1. **Immutable Borrowing**: Multiple variables can have immutable references (`&T`) to a value, allowing them to read but not modify the value. Immutable borrows can coexist with other immutable borrows.

### Example

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

2. **Mutable Borrowing**: Only one variable can have a mutable reference (`&mut T`) to a value at a time. Mutable borrowing ensures exclusive access to modify the value and prevents data races by disallowing concurrent mutable references.

### Example 

```rust
fn main() {
    let mut x = 5; // Declare a mutable variable x and assign it the value 5

    let y = &mut x; // Mutable borrow of x using a mutable reference

    *y += 1; // Modify the value through the mutable reference

    println!("x: {}", x); // Prints "x: 6"
}
```

3. **Borrowing and Scopes**: Borrowing is limited to the scope in which the borrow is valid. When the borrow goes out of scope, the value can be borrowed again or returned to the owner.

### Example

``` rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

```
In above example, we dont wanted to get the ownership of the s1, then we use the &(refrence) of that string, so that ownership does not gets transferred.


### Note
We cannot use two or more that two mutable references or immutable reference with mutable reference. This is beacuse of the reason that if mutable reference changes/updates the value of the variable then there is no way to sync both of the references, which may result in corrupt data.


## Lifetimes

Lifetimes are a key part of Rust's ownership system, ensuring that references remain valid for the duration they are used. Lifetimes define the scope during which a reference is valid and prevent references to values that no longer exist.

1. **Explicit Lifetime Annotations**: Rust uses lifetime annotations (`'a`) to describe the relationships between references, ensuring that references do not outlive the values they refer to.

2. **Lifetime Elision**: Rust's lifetime elision rules automatically determine lifetimes in many cases, reducing the need for explicit lifetime annotations. These rules make code more concise and readable.

## Conclusion

Understanding ownership, borrowing, and lifetimes is crucial for writing safe and efficient Rust code. By enforcing strict ownership rules, Rust eliminates many common bugs and ensures memory safety without the need for a garbage collector. This README file provided an overview of the ownership concept in Rust, but there is much more to explore in the Rust documentation and other learning resources.

