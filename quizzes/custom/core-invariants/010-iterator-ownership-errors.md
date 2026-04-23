Answer: error
Difficulty: 2
Tags: iterators, ownership, move-semantics, for-loop

# Hint

`for x in v` is sugar for `v.into_iter()`, which moves `v` — you cannot use it after the loop.

# Explanation

This program demonstrates the **Iterator Ownership Error** failure mode.

```rust
fn main() {
    let v = vec![1, 2, 3];
    for x in v {           // v is moved into the iterator
        println!("{}", x);
    }
    println!("{:?}", v);   // ERROR: use of moved value `v`
}
```

The learner might think: *"After the loop finishes, `v` should still be there — I only printed the elements."*

In Rust, `for x in v` is desugared to:
```rust
let mut iter = v.into_iter(); // v is moved here
while let Some(x) = iter.next() {
    println!("{}", x);
}
```

`into_iter()` **takes ownership** of `v`, moving it into the iterator. Once the loop ends, `v` is gone — the iterator consumed it. This is the **Iterator Consumption** invariant in action: iterating by value transfers ownership.

**Why this invariant exists**  
Ownership of the collection must move into the iterator so the iterator can yield elements by value (not by reference). If `v` remained valid after the loop, two pieces of code could own the same heap memory — a double-free waiting to happen.

**Fix option 1: iterate by reference (keep `v` alive)**
```rust
for x in &v {               // borrows v — equivalent to v.iter()
    println!("{}", x);
}
println!("{:?}", v);        // OK — v was never moved
```

**Fix option 2: use `.iter()` explicitly**
```rust
for x in v.iter() {
    println!("{}", x);
}
println!("{:?}", v);        // OK
```

Use `for x in v` (consuming) only when you truly want to take ownership of the elements and do not need the collection afterward.
