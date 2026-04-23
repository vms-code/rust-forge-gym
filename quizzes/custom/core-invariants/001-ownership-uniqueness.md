Answer: 1
Difficulty: 1
Warnings:
Tags: ownership, move-semantics

# Hint

Only one `Drop` implementation runs even though we created two bindings (`a` and `b`).

# Explanation

The program defines a struct `S` whose `Drop` implementation prints its inner value when the value is destroyed.

We create `let a = S(1);` and then do `let b = a;`.

This is a **move**. Because `S` is not `Copy`, ownership of the value is transferred from `a` to `b`. The original binding `a` is invalidated by the compiler.

This demonstrates Rust’s **Ownership Uniqueness** invariant: every value has exactly one owner at a time. Assigning or passing a non-`Copy` value *moves* it, making the old binding unusable. This guarantees that resources (heap data, files, sockets, etc.) are freed exactly once when the single owner is dropped.

At the end of `main()`, only `b` goes out of scope, so the `Drop` impl runs once and prints `1`.

**Invalid example (use after move):**
```rust
let a = String::from("hello");
let b = a;
println!("{}", a);  // ERROR: use of moved value `a`
```

The compiler enforces this rule at compile time to prevent double-frees and use-after-free bugs — one of the core reasons Rust can be both safe and fast without a garbage collector.

This invariant underlies function arguments, pattern matching, and almost every part of the language.