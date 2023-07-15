# Mod

The `mod` feature in Rust is used to create and organize modules, which are used to group related code together. Modules provide a way to organize code into logical units, improve code organization and readability, and allow for better code reuse and abstraction.

## Purpose of Modules

Modules serve multiple purposes in Rust:

- Code Organization: Modules allow you to organize your code into smaller, more manageable units. By grouping related functionality together, you can easily locate and work with specific parts of your codebase.

- Encapsulation: Modules provide encapsulation, allowing you to hide implementation details and expose only the necessary interfaces to other parts of your code. This promotes better code design and reduces dependencies between different components.

- Code Reusability: Modules enable code reuse by allowing you to define reusable components and import them into different parts of your codebase. This promotes modularity and reduces the need for duplicating code.

- Namespace Separation: Modules provide a way to separate namespaces, allowing you to avoid naming conflicts between different parts of your code. Each module forms its own namespace, ensuring that names within a module do not clash with names from other modules.

## Defining Modules

Modules in Rust are defined using the `mod` keyword followed by the module name. The module name should be in `snake_case` convention and should reflect the purpose or functionality of the module.

```rust
mod module_name {
// Module contents go here
}
```


## Using Modules

Once you have defined a module, you can use it by either referencing it with its full path or by bringing it into scope using the `use` keyword. Using the full path allows you to access items from the module directly, while using `use` brings the items into the current scope for easier access.

```rust
// Using the full path
module_name::function_name();

// Bringing items into scope
use module_name::function_name;
function_name();
```


## Visibility and Privacy

By default, items within a module are private and cannot be accessed from outside the module. To make an item within a module accessible to other parts of your codebase, you can use the `pub` keyword to mark it as public.

```rust
mod module_name {
pub fn public_function() {
// Public function implementation
}
```