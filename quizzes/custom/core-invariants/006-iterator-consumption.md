Answer: 123
Difficulty: 2
Tags: iterators, ownership, consumption

# Hint

After the `for` loop finishes, the original vector can no longer be used.

# Explanation

This program demonstrates Rust’s **Iterator Consumption** invariant.

```rust
let v = vec![1, 2, 3];
for x in v {          // v is moved into the iterator
    print!("{}", x);
}
// v is now consumed — you cannot use it again
```

In Rust, a `for` loop is syntax sugar for calling `into_iter()` (or `iter()`/`iter_mut()`) on the collection. The `into_iter()` method **takes ownership** of the vector (`v` is moved), turning it into an iterator that yields the elements by value.

Once the iterator is consumed (or the loop ends), the original collection is no longer accessible by name. This is the same ownership rule we saw earlier: non-`Copy` values are moved.

**Common related example (the error learners often hit):**
```rust
let v = vec![1, 2, 3];
for x in v {
    println!("{}", x);
}
println!("{:?}", v); // ERROR: use of moved value `v`
```

**Correct ways to keep using the collection:**
- Borrow with `for x in &v { ... }` (iterates by reference)
- Or `for x in v.iter() { ... }`

**Why this invariant exists**  
Iterators are ordinary values. Consuming them prevents you from accidentally using a partially-exhausted or invalidated iterator, and it ties directly into Rust’s ownership system. It guarantees that once you’ve transferred ownership into an iterator, the original data can’t be mutated or read in conflicting ways.

This rule appears in almost every use of `for` loops, `.map()`, `.filter()`, `.collect()`, etc. Understanding it is essential for writing idiomatic Rust.
