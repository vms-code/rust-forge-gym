Answer: error
Difficulty: 1
Tags: borrowing, borrow-checker, aliasing

# Hint

You cannot create a mutable reference while any immutable reference to the same data is still active.

# Explanation

This program triggers Rust’s most famous borrow-checker error.

```rust
let mut x = 5;
let r1 = &x;           // immutable borrow starts
let r2 = &mut x;       // ERROR: cannot borrow `x` as mutable because it is also borrowed as immutable
println!("{}, {}", r1, r2);
```

Rust’s **Aliasing vs. Mutability** invariant states: at any moment you may have *either*:
- any number of immutable references (`&T`), **or**
- exactly one mutable reference (`&mut T`)

You can never have both at the same time.

**Why this rule exists**  
It guarantees memory safety without a garbage collector:
- No data races (multiple readers + writer = race condition)
- No iterator invalidation (you can’t mutate a collection while iterating over it)
- No unexpected changes while someone else is reading the value

The compiler enforces this **statically** at compile time.

**Correct version (using scopes to end the immutable borrow):**

```rust
fn main() {
    let mut x = 5;

    {
        let r1 = &x;           // immutable borrow
        let r2 = &x;           // another immutable borrow – allowed
        println!("{}, {}", r1, r2);
    } // ← immutable borrows end here

    let r3 = &mut x;           // mutable borrow now allowed
    *r3 += 1;
    println!("{}", r3);        // prints 6
}
```

This is the same rule demonstrated in the classic example from the source material:

```rust
let mut x = 5;
let r1 = &x;
let r2 = &mut x; // ERROR
```

**Takeaway**  
The borrow checker is not being “picky” — it is protecting one of Rust’s fundamental safety guarantees. When you see this error, the fix is almost always to shrink the lifetime of the immutable borrow (with a block, by using the value immediately, or by refactoring to avoid overlapping borrows).

This invariant is the reason Rust code can be both fearless and extremely fast.
