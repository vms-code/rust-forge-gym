Answer: 12
Difficulty: 2
Tags: move-semantics, copy

# Hint

`i32` can be used after assignment, but `String` cannot.

# Explanation

This program demonstrates Rust’s **Move Semantics & `Copy` Types** invariant.

```rust
let x: i32 = 1;
let y = x;          // Copy: x is still usable
print!("{}", x);    // prints 1

let a = String::from("2");
let b = a;          // Move: a is now invalid
print!("{}", b);    // prints 2
```

- Types that implement the `Copy` trait (like all primitives: `i32`, `bool`, `char`, tuples of `Copy` types, etc.) are **implicitly copied** on assignment or when passed by value. The original binding remains fully usable.
- Non-`Copy` types (like `String`, `Vec<T>`, structs containing non-`Copy` fields, etc.) are **moved** by default. The original binding is invalidated — this is what prevents use-after-free bugs.

**Valid example (Copy type):**
```rust
let a = 5u32;
let b = a;         // copy
println!("{}", a); // OK
```

**Invalid example (non-Copy type):**
```rust
let s1 = String::from("foo");
let s2 = s1;       // move
println!("{}", s1); // ERROR: use of moved value `s1`
```

The compiler enforces this rule automatically. This is why you sometimes need `.clone()` (expensive) or borrowing (`&T`) when you want to keep the original value.

This invariant is one of the foundations of Rust’s ownership system and is the reason you see “use of moved value” errors so often when learning the language.
