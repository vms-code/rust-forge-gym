Answer: [0, 2, 4]
Difficulty: 1
Tags: iterators, filter, collect, functional-style

# Hint

`filter` keeps only elements for which the closure returns `true`; `collect` materialises the iterator into a `Vec`.

# Explanation

This program demonstrates the **Imperative Collection Building → Iterator Chains** transformation pattern.

```rust
fn main() {
    let evens: Vec<_> = (0..6).filter(|x| x % 2 == 0).collect();
    println!("{:?}", evens); // [0, 2, 4]
}
```

**Naive approach — imperative `for` loop with `push`:**
```rust
let mut evens = Vec::new();
for x in 0..6 {
    if x % 2 == 0 {
        evens.push(x);
    }
}
```

The naive version manually builds the vector with mutable state. While perfectly correct, it is more verbose and harder to read at a glance.

**Idiomatic approach — iterator combinators**  

| Step | What it does |
|------|------|
| `(0..6)` | Creates a `Range<i32>` iterator yielding 0, 1, 2, 3, 4, 5 |
| `.filter(\|x\| x % 2 == 0)` | Keeps only even values (lazy — no work done yet) |
| `.collect()` | Drives the iterator to completion, collecting into `Vec<_>` |

The type annotation `Vec<_>` lets the compiler infer the element type (`i32`) from context.

**Why iterator chains are preferred**  
- **No mutable state** — `evens` is declared without `mut`
- **Declarative** — reads as "the even numbers in 0..6"
- **Composable** — chains like `.filter(...).map(...).take(n).collect()` build complex pipelines without nested loops
- **Lazy** — combinators like `filter` and `map` produce no allocations until `collect` (or another consuming adaptor) is called

**More combinator examples:**
```rust
// squares of odd numbers
let odd_squares: Vec<_> = (0..10)
    .filter(|x| x % 2 != 0)
    .map(|x| x * x)
    .collect();

// sum of all elements
let total: i32 = (0..6).filter(|x| x % 2 == 0).sum();
```
