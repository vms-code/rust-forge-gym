Answer: error
Difficulty: 2
Tags: borrowing, aliasing, mutability

# Hint

An active immutable borrow and a mutable borrow of the same value cannot coexist.

# Explanation

This program demonstrates the **Immutable vs. Mutable Borrow Collision** failure mode.

```rust
fn main() {
    let mut a = 5;
    let r1 = &a;       // immutable borrow — starts here
    let r2 = &mut a;   // ERROR: a mutable borrow while r1 is still live
    println!("{}, {}", r1, r2);
}
```

The learner might reason: *"I have one immutable borrow and then I take a mutable borrow — they're separate variables so it should be fine."*

In reality, Rust's borrow checker enforces the **Aliasing XOR Mutability** invariant: **at most one `&mut T` reference OR any number of `&T` references may exist at the same time — never both**.

Here `r1` is still live (it is used in the `println!` on the next line), so the compiler rejects `&mut a` at the point it is created. The lifetimes of `r1` and `r2` overlap.

**Why this rule exists**  
Allowing a mutable reference to exist alongside an immutable one would mean a reader could observe the data changing under its feet. This is a data race in disguise — even on a single thread, it violates the guarantee that `&T` gives access to *stable* data. The rule prevents iterator-invalidation bugs and other subtle memory-safety issues.

**Fix: end the immutable borrow first**
```rust
fn main() {
    let mut a = 5;
    {
        let r1 = &a;
        println!("{}", r1); // r1's lifetime ends here
    }
    let r2 = &mut a;        // now safe — no active immutable borrows
    *r2 += 1;
    println!("{}", r2);
}
```

Or, with NLL (Non-Lexical Lifetimes), simply make sure `r1` is not used after the point where you take `r2`:
```rust
let r1 = &a;
println!("{}", r1); // last use of r1 — its lifetime ends here
let r2 = &mut a;    // OK
*r2 += 1;
```
