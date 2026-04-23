Answer: error
Difficulty: 2
Tags: lifetimes, references, dangling

# Hint

A reference returned from a function must not point to a local variable that is dropped when the function ends.

# Explanation

This program demonstrates Rust’s **Lifetime/Validity of References** invariant.

```rust
fn dangle() -> &String {
    let s = String::from("hello");
    &s  // ERROR: `s` does not live long enough
}
```

Rust guarantees that every reference (`&T` or `&mut T`) is always valid — it can never point to memory that has been freed (a *dangling reference*).

Here the function `dangle` tries to return a reference to the local `String` `s`. But `s` is destroyed at the end of the function, so the returned reference would point to invalid memory. The compiler rejects this code with a lifetime error.

**Valid version (return ownership instead of a reference):**
```rust
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // ownership is moved out — no dangling reference
}

fn main() {
    let owned = no_dangle();
    println!("{}", owned);  // OK
}
```

**Why this rule exists**  
Allowing dangling references would lead to use-after-free bugs, which are a major source of memory safety vulnerabilities in other languages. Rust’s borrow checker enforces this statically so you never have to worry about it at runtime.

This invariant appears everywhere: function parameters, return values, structs that hold references, and even in `std` types like slices (`&[T]`).

**Takeaway**  
When you see “does not live long enough” or “cannot return reference to local data”, the fix is almost always one of these:
- Return an owned value instead of a reference
- Take the reference as a parameter from the caller (`fn foo(s: &String)`)
- Store the data in a longer-lived scope

This is one of Rust’s most important safety guarantees.
