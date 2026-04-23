Answer: [0, 2, 4]
Difficulty: 1
Tags: iterators, for-loop, iter_mut, enumerate

# Hint

`iter_mut()` yields mutable references to each element; `enumerate()` pairs each with its index.

# Explanation

This program demonstrates the **Indexing Loops → Iterator Loops** transformation pattern.

```rust
fn main() {
    let mut v = vec![0; 3];
    for (i, elem) in v.iter_mut().enumerate() {
        *elem = i * 2; // i = 0,1,2 → elem = 0,2,4
    }
    println!("{:?}", v); // [0, 2, 4]
}
```

**Naive approach — index-based loop:**
```rust
let mut v = vec![0; 3];
for i in 0..v.len() {
    v[i] = i * 2;
}
```

The naive version uses manual indexing (`v[i]`), which:
- Performs a bounds check on every access (runtime overhead)
- Requires knowing `v.len()` upfront
- Is easy to get wrong (off-by-one errors, etc.)

**Idiomatic approach — `iter_mut().enumerate()`**  
`iter_mut()` borrows `v` mutably and yields `&mut T` references to each element. `enumerate()` wraps each reference in a `(index, &mut T)` tuple. The `*elem = ...` dereferences the mutable reference to write through it.

This approach is:
- **Safe** — Rust's type system guarantees you can't access `v` in another way while `iter_mut` holds a mutable borrow
- **Clear** — the intent ("for each element by index, do X") is explicit
- **Bounds-check-free** — iterators never go out of bounds by construction

**Related patterns:**
```rust
// Read-only access with index
for (i, elem) in v.iter().enumerate() { ... }

// Consume values with index
for (i, elem) in v.into_iter().enumerate() { ... }
```
