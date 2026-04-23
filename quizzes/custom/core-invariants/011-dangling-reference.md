Answer: error
Difficulty: 2
Tags: lifetimes, references, dangling, borrow-checker

# Hint

A reference cannot outlive the value it points to — returning a reference to a local variable is always rejected.

# Explanation

This program demonstrates the **Dangling Reference** failure mode.

```rust
fn make_ref<'a>() -> &'a String {
    let s = String::from("hello");
    &s  // ERROR: `s` does not live long enough
}

fn main() {
    let _r = make_ref();
}
```

The learner might think: *"I'm just returning a reference — Rust will figure out the lifetimes."*

In reality, `s` is a **local variable** that lives only for the duration of `make_ref`. When the function returns, `s` is dropped and its memory is freed. Returning `&s` would give the caller a reference to freed memory — a classic **dangling pointer**. The lifetime parameter `'a` on the return type promises that the reference will be valid for *some* caller-chosen lifetime, but there is no value in scope that can back that promise — Rust's borrow checker detects this and rejects the code.

**Why this invariant exists**  
Rust's core guarantee is that references are **always valid**. A dangling reference — one pointing to freed memory — is one of the most common sources of undefined behaviour in C and C++. The lifetime system encodes this guarantee at compile time: a reference can never outlive its referent.

**Fix option 1: return an owned value instead of a reference**
```rust
fn make_string() -> String {
    let s = String::from("hello");
    s  // ownership is moved to the caller — no dangling reference possible
}
```

**Fix option 2: accept a reference from the caller and return it**
```rust
fn first_word<'a>(s: &'a str) -> &'a str {
    // the returned reference is tied to the *input* reference,
    // which the caller is responsible for keeping alive
    s.split_whitespace().next().unwrap_or("")
}
```

The rule of thumb: if a function returns a reference, it must come from one of the function's inputs (or from a `'static` source like a string literal), never from a local variable.
