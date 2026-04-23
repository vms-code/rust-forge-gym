Answer: Hello, Alice!
Hello, Alice!
name is still: Alice
Difficulty: 1
Tags: borrowing, cloning, ownership, functions

# Hint

Passing `&name` borrows the `String` — the caller keeps ownership and can use it again afterward.

# Explanation

This program demonstrates the **Cloning → Borrowing** transformation pattern.

```rust
fn greet(name: &String) {
    println!("Hello, {}!", name);
}

fn main() {
    let name = String::from("Alice");
    greet(&name); // borrows — name is still usable
    greet(&name); // borrows again — still valid
    println!("name is still: {}", name);
}
```

**Naive approach — taking ownership (or cloning to work around it):**
```rust
fn greet(name: String) { // takes ownership
    println!("Hello, {}!", name);
}

fn main() {
    let name = String::from("Alice");
    greet(name.clone()); // must clone to keep `name` alive
    greet(name.clone()); // clone again
    println!("{}", name);
}
```

The naive version must clone `name` before each call because `greet` consumes it. Every `.clone()` performs a heap allocation — wasteful when we only need to *read* the value.

**Idiomatic approach — borrow with `&String` (or better, `&str`)**  
By changing the parameter to `&String`, the function only borrows the data. The caller retains ownership and can pass the same value as many times as needed without any copying.

In practice, idiomatic Rust prefers `&str` over `&String` for string parameters:
```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}
// &String automatically coerces to &str via Deref
```

**Why this transformation matters**  
Unnecessary clones are a common beginner mistake in Rust. They arise when ownership requirements are misunderstood. Borrowing is nearly always the right choice for read-only access: it is zero-cost (no heap allocation), expressive (the type signature signals "I only read this"), and keeps the API flexible (callers can pass `&String`, `&str`, or even `&'static str`).
