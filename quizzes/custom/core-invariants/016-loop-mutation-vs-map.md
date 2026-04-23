Answer: [2, 4, 6]
Difficulty: 1
Tags: iterators, map, collect, functional-style, transformation

# Hint

`.map(|n| n * 2)` applies a transformation to every element; `.collect()` gathers the results into a new `Vec`.

# Explanation

This program demonstrates the **Direct Loop Mutation → Iterator with `map`** transformation pattern.

```rust
fn main() {
    let nums = vec![1, 2, 3];
    let doubles: Vec<_> = nums.into_iter().map(|n| n * 2).collect();
    println!("{:?}", doubles); // [2, 4, 6]
}
```

**Naive approach — `for` loop with `push`:**
```rust
let nums = vec![1, 2, 3];
let mut doubles = Vec::new();
for n in nums {
    doubles.push(n * 2);
}
```

The naive version requires a mutable `doubles` vector and an explicit loop. Every element must be pushed one by one. The `map` version expresses the same idea — "transform every element" — in a single declarative line.

**Idiomatic approach — `.map().collect()`**  

| Step | What it does |
|------|------|
| `nums.into_iter()` | Consumes `nums`, turning it into an iterator of owned `i32` values |
| `.map(\|n\| n * 2)` | Lazily applies the closure to each element (no allocation yet) |
| `.collect()` | Drives the iterator and collects results into a `Vec<i32>` |

**Why `.map()` is preferred**  
- **No mutable state** — `doubles` does not need `mut`
- **Single responsibility** — the closure expresses only the transformation; the plumbing (iteration, collection) is handled by the adaptor chain
- **Composable** — `map` chains naturally with other adaptors:

```rust
// double only the odd numbers
let result: Vec<_> = nums.iter()
    .filter(|&&n| n % 2 != 0)
    .map(|&n| n * 2)
    .collect();
```

**`iter()` vs `into_iter()` with `map`**  
- `nums.iter().map(|n| n * 2)` — borrows `nums`, closure receives `&i32`, must deref: `|&n| n * 2` or `|n| n * 2` (auto-deref via `Copy`)
- `nums.into_iter().map(|n| n * 2)` — consumes `nums`, closure receives `i32` directly

For `Copy` types like `i32` the distinction rarely matters; for `String` or other non-`Copy` types, `into_iter()` lets you transform owned values without cloning.
