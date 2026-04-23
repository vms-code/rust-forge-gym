Answer: 42
Difficulty: 1
Tags: pattern-matching, if-let, option, ergonomics

# Hint

`if let Some(x) = opt` binds the inner value only when the pattern matches — `None` is silently ignored.

# Explanation

This program demonstrates the **Manual Matching → `if let`** transformation pattern.

```rust
fn main() {
    let opt: Option<i32> = Some(42);
    if let Some(x) = opt {
        println!("{}", x); // prints 42
    }
}
```

**Naive approach — full `match` with an empty arm:**
```rust
match opt {
    Some(x) => println!("{}", x),
    None => (), // nothing to do — boilerplate
}
```

Both versions compile and behave identically. The `match` version is exhaustive (it handles `None`), but when you have nothing to do for the other variants, the `None => ()` arm is pure noise.

**Idiomatic approach — `if let`**  
`if let` is syntactic sugar for a `match` that only cares about one pattern:
```rust
// Exactly equivalent to the match above
if let Some(x) = opt {
    println!("{}", x);
}
```

You can also chain an `else` branch when you *do* need to handle the missing case:
```rust
if let Some(x) = opt {
    println!("got {}", x);
} else {
    println!("nothing");
}
```

**The `matches!` macro — for boolean checks only**  
When you just need a `bool` (no binding), `matches!` is even terser:
```rust
let is_some = matches!(opt, Some(_)); // true
```

**Why this pattern matters**  
- Reduces visual noise compared to `match` with throwaway arms
- Works with any pattern, not just `Option`: `if let Ok(v) = result { ... }`, `if let MyEnum::Variant(data) = val { ... }`
- Pair it with `while let` for loops that consume an iterator or a channel:
```rust
while let Some(x) = iter.next() {
    println!("{}", x);
}
```
