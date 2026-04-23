Answer: error
Difficulty: 1
Tags: pattern-matching, exhaustiveness

# Hint

The compiler requires every possible variant of an `enum` to be handled in a `match`.

# Explanation

This program demonstrates Rust’s **Pattern Matching Exhaustiveness** invariant.

```rust
enum E { A, B }

fn main() {
    match E::A {
        E::A => println!("A"),
        // missing E::B arm → non-exhaustive
    }
}
```

Rust guarantees that a `match` (or any other pattern binding) must cover **all possible cases**. The compiler performs exhaustive checking at compile time. Omitting a variant (or using `_` for the rest) causes a compile error: “non-exhaustive patterns”.

**Valid example (exhaustive):**
```rust
enum E { A, B }

fn main() {
    match E::A {
        E::A => println!("A"),
        E::B => println!("B"),
    }
}
```

**Why this rule exists**  
It prevents runtime surprises and “match fell through” bugs that exist in other languages. A non-exhaustive `match` could silently do nothing or return the wrong value. Exhaustiveness ensures every `match` expression always produces a value and that the programmer has consciously considered every case.

You can also use the catch-all `_` arm when you don’t care about the remaining variants:
```rust
match e {
    E::A => println!("A"),
    _ => println!("something else"),
}
```

This invariant applies to `match`, `if let`, function parameters with patterns, `let` destructuring, and even `for` loops over `Option`/`Result`. It is one of Rust’s strongest compile-time correctness guarantees.
