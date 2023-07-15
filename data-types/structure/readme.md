# Struct

The `struct` keyword in Rust is used to define a custom data type known as a structure or struct. A struct allows you to create a composite type by grouping together multiple related values with different data types into a single unit.

## Purpose of Structs

Structs serve multiple purposes in Rust:

- Data Organization: Structs allow you to organize related data into a cohesive unit, making it easier to manage and manipulate the data as a whole.

- Data Abstraction: Structs provide a way to abstract complex data structures and represent them in a more understandable and intuitive manner.

- Code Reusability: Structs enable code reuse by defining a blueprint that can be used to create multiple instances of the same structure with different data values.

- Data Encapsulation: Structs support encapsulation by allowing you to specify the visibility of its fields, which determines whether they can be accessed from outside the struct.

## Struct Syntax

The syntax for defining a struct in Rust is as follows:

```rust
struct StructName {
field1: Type1,
field2: Type2,
// ...
}
```


- StructName: The name given to the struct type. It should be in PascalCase, starting with an uppercase letter.

- field1, field2, etc.: The fields or data members of the struct. Each field is defined with a name and its corresponding data type.

## Field Accessibility

By default, the fields of a struct are private and can only be accessed within the same module. To make the fields accessible from outside the struct or module, you can use the `pub` keyword to mark them as public.


```rust
pub struct StructName {
pub field1: Type1,
pub field2: Type2,
// ...
}
```

In this example, the field1 and field2 are marked as `pub` and can be accessed from outside the struct or module.

## Creating Instances of Structs

To create an instance of a struct, you use the `StructName { field1: value1, field2: value2, ... }` syntax, providing values for each field.

```rust
let instance = StructName { field1: value1, field2: value2, ... };
```

## Accessing Struct Fields

You can access the fields of a struct using dot notation (.) followed by the field name.

```rust
let value = instance.field1;
```

## Summary

Structs in Rust provide a way to define custom data types by grouping related values into a single unit. They are used for data organization, abstraction, code reusability, and encapsulation. Understanding how to define and work with structs is essential for building complex data structures and organizing your code effectively.\


