# RUST.md - Common Rust Errors and Best Practices Guide

A comprehensive guide to avoiding common Rust errors and writing idiomatic Rust code.

## Table of Contents
1. [Ownership and Borrowing Errors](#ownership-and-borrowing-errors)
2. [Lifetime Errors](#lifetime-errors)
3. [Type System Errors](#type-system-errors)
4. [Async/Await Pitfalls](#asyncawait-pitfalls)
5. [Error Handling Best Practices](#error-handling-best-practices)
6. [Performance Pitfalls](#performance-pitfalls)
7. [Common Clippy Warnings](#common-clippy-warnings)
8. [Testing Best Practices](#testing-best-practices)

## Ownership and Borrowing Errors

### 1. Cannot Move Out of Borrowed Content

**Error Example:**
```rust
fn process_vec(v: &Vec<String>) {
    let first = v[0]; // Error: cannot move out of index
}
```

**Solution:**
```rust
fn process_vec(v: &Vec<String>) {
    let first = &v[0]; // Borrow instead
    // Or clone if you need ownership
    let first_owned = v[0].clone();
}
```

### 2. Cannot Borrow as Mutable More Than Once

**Error Example:**
```rust
let mut data = vec![1, 2, 3];
let r1 = &mut data;
let r2 = &mut data; // Error: second mutable borrow
```

**Solution:**
```rust
let mut data = vec![1, 2, 3];
{
    let r1 = &mut data;
    // Use r1
} // r1 goes out of scope
let r2 = &mut data; // Now this is fine

// Or use split_at_mut for slices
let (left, right) = data.split_at_mut(1);
```

### 3. Use After Move

**Error Example:**
```rust
let s = String::from("hello");
let s2 = s; // s is moved
println!("{}", s); // Error: use after move
```

**Solution:**
```rust
// Clone if you need both
let s = String::from("hello");
let s2 = s.clone();
println!("{}", s); // Works fine

// Or use references
let s = String::from("hello");
let s2 = &s;
println!("{}", s); // Original still accessible
```

## Lifetime Errors

### 1. Lifetime Parameter Required

**Error Example:**
```rust
struct Container {
    data: &str, // Error: missing lifetime
}
```

**Solution:**
```rust
struct Container<'a> {
    data: &'a str,
}

// Or use owned data
struct Container {
    data: String,
}
```

### 2. Lifetime May Not Live Long Enough

**Error Example:**
```rust
fn longest(x: &str, y: &str) -> &str { // Error: missing lifetime
    if x.len() > y.len() { x } else { y }
}
```

**Solution:**
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

### 3. Dangling References

**Error Example:**
```rust
fn dangling() -> &String { // Error: missing lifetime
    let s = String::from("hello");
    &s // s is dropped at end of function
}
```

**Solution:**
```rust
fn not_dangling() -> String {
    String::from("hello") // Return owned value
}
```

## Type System Errors

### 1. Type Annotations Needed

**Error Example:**
```rust
let numbers = vec![1, 2, 3];
let parsed = numbers.iter().map(|n| n.to_string()).collect(); // Error: type needed
```

**Solution:**
```rust
let numbers = vec![1, 2, 3];
let parsed: Vec<String> = numbers.iter().map(|n| n.to_string()).collect();
// Or use turbofish
let parsed = numbers.iter().map(|n| n.to_string()).collect::<Vec<String>>();
```

### 2. Trait Not Implemented

**Error Example:**
```rust
struct Point { x: i32, y: i32 }
let p1 = Point { x: 1, y: 2 };
let p2 = Point { x: 3, y: 4 };
if p1 == p2 {} // Error: PartialEq not implemented
```

**Solution:**
```rust
#[derive(Debug, PartialEq)]
struct Point { x: i32, y: i32 }
// Or implement manually
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
```

### 3. Type Mismatch in Match Arms

**Error Example:**
```rust
let value = match some_option {
    Some(x) => x,
    None => "default", // Error: different types
};
```

**Solution:**
```rust
// Ensure all arms return same type
let value = match some_option {
    Some(x) => x.to_string(),
    None => "default".to_string(),
};
```

## Async/Await Pitfalls

### 1. Cannot Be Sent Between Threads Safely

**Error Example:**
```rust
use std::rc::Rc;
async fn bad_async() {
    let rc = Rc::new(5); // Rc is not Send
    some_async_fn().await;
    println!("{}", rc);
}
```

**Solution:**
```rust
use std::sync::Arc;
async fn good_async() {
    let arc = Arc::new(5); // Arc is Send
    some_async_fn().await;
    println!("{}", arc);
}
```

### 2. Holding Lock Across Await Point

**Error Example:**
```rust
use tokio::sync::Mutex;
async fn bad_lock(mutex: &Mutex<i32>) {
    let mut guard = mutex.lock().await;
    some_async_operation().await; // Bad: holding lock across await
    *guard += 1;
}
```

**Solution:**
```rust
async fn good_lock(mutex: &Mutex<i32>) {
    {
        let mut guard = mutex.lock().await;
        *guard += 1;
    } // Lock released before await
    some_async_operation().await;
}
```

### 3. Future Not Send

**Error Example:**
```rust
async fn process_data(data: &str) -> Result<String, Box<dyn Error>> {
    // Error when spawning: future is not Send
    Ok(data.to_string())
}
```

**Solution:**
```rust
async fn process_data(data: String) -> Result<String, Box<dyn Error + Send + Sync>> {
    Ok(data)
}
```

## Error Handling Best Practices

### 1. Don't Unwrap in Production Code

**Bad:**
```rust
let file_content = std::fs::read_to_string("file.txt").unwrap(); // Panics on error
```

**Good:**
```rust
// Use ? operator
fn read_file() -> Result<String, std::io::Error> {
    let file_content = std::fs::read_to_string("file.txt")?;
    Ok(file_content)
}

// Or handle explicitly
match std::fs::read_to_string("file.txt") {
    Ok(content) => process(content),
    Err(e) => eprintln!("Failed to read file: {}", e),
}
```

### 2. Custom Error Types

**Good Practice:**
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),
    
    #[error("Custom error: {msg}")]
    Custom { msg: String },
}
```

### 3. Error Context

**Good Practice:**
```rust
use anyhow::{Context, Result};

fn process_config() -> Result<Config> {
    let content = std::fs::read_to_string("config.json")
        .context("Failed to read config file")?;
    
    let config: Config = serde_json::from_str(&content)
        .context("Failed to parse config JSON")?;
    
    Ok(config)
}
```

## Performance Pitfalls

### 1. Unnecessary Cloning

**Bad:**
```rust
fn process_string(s: String) -> String {
    s.clone() // Unnecessary clone, s is already owned
}
```

**Good:**
```rust
fn process_string(s: String) -> String {
    s // Just return the owned value
}

// Or use references when possible
fn process_string(s: &str) -> String {
    s.to_string() // Only allocate when needed
}
```

### 2. Collecting When Not Needed

**Bad:**
```rust
let sum: i32 = vec![1, 2, 3, 4, 5]
    .iter()
    .map(|x| x * 2)
    .collect::<Vec<_>>() // Unnecessary allocation
    .iter()
    .sum();
```

**Good:**
```rust
let sum: i32 = vec![1, 2, 3, 4, 5]
    .iter()
    .map(|x| x * 2)
    .sum(); // Direct sum without collecting
```

### 3. String Concatenation in Loops

**Bad:**
```rust
let mut result = String::new();
for i in 0..1000 {
    result = result + &i.to_string(); // Creates new String each time
}
```

**Good:**
```rust
let mut result = String::new();
for i in 0..1000 {
    result.push_str(&i.to_string()); // Modifies in place
}

// Or use with_capacity
let mut result = String::with_capacity(4000);
for i in 0..1000 {
    use std::fmt::Write;
    write!(&mut result, "{}", i).unwrap();
}
```

## Common Clippy Warnings

### 1. Needless Borrow

**Warning:**
```rust
let s = String::from("hello");
let len = (&s).len(); // Warning: needless borrow
```

**Fix:**
```rust
let s = String::from("hello");
let len = s.len(); // Auto-deref handles it
```

### 2. Single Match Instead of If Let

**Warning:**
```rust
match some_option {
    Some(x) => println!("{}", x),
    None => {},
}
```

**Fix:**
```rust
if let Some(x) = some_option {
    println!("{}", x);
}
```

### 3. Manual Implementation of Default

**Warning:**
```rust
impl Default for MyStruct {
    fn default() -> Self {
        MyStruct {
            field1: 0,
            field2: String::new(),
        }
    }
}
```

**Fix:**
```rust
#[derive(Default)]
struct MyStruct {
    field1: i32,
    field2: String,
}
```

## Testing Best Practices

### 1. Test Organization

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_functionality() {
        // Arrange
        let input = "test";
        
        // Act
        let result = process(input);
        
        // Assert
        assert_eq!(result, "expected");
    }
    
    #[test]
    #[should_panic(expected = "invalid input")]
    fn test_panic_on_invalid_input() {
        process("");
    }
}
```

### 2. Async Tests

```rust
#[tokio::test]
async fn test_async_function() {
    let result = async_function().await;
    assert_eq!(result, expected);
}
```

### 3. Property-Based Testing

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_parse_doesnt_crash(s in "\\PC*") {
        let _ = parse_string(&s); // Should not panic
    }
}
```

## Common Redis-Specific Issues

### 1. Type Annotations for Redis Commands

**Error:**
```rust
conn.set("key", "value").await?; // Error: type annotations needed
```

**Fix:**
```rust
// Specify return type
let _: () = conn.set("key", "value").await?;
// Or use turbofish
conn.set::<_, _, ()>("key", "value").await?;
```

### 2. Handling Optional Redis Values

**Good Practice:**
```rust
// Use Option for potentially missing keys
let value: Option<String> = conn.get("key").await?;
match value {
    Some(v) => println!("Found: {}", v),
    None => println!("Key not found"),
}
```

## General Best Practices

1. **Use `clippy` regularly**: `cargo clippy -- -D warnings`
2. **Format code**: `cargo fmt`
3. **Document public APIs**: Use `///` for public items
4. **Use `#[must_use]`**: For functions that return important values
5. **Prefer `&str` over `&String`**: More flexible function parameters
6. **Use `impl Trait`**: For simpler function signatures
7. **Enable all compiler warnings**: `#![warn(missing_docs, missing_debug_implementations)]`

## Useful Compiler Flags

```toml
# In Cargo.toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

## Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Effective Rust](https://www.lurklurk.org/effective-rust/)
- [Common Rust Lifetime Misconceptions](https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md)