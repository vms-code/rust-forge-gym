Question: Which of the following best describes the concept of interior mutability in Rust?
Answer: A
Type: multiple-choice
Difficulty: 1
Tags: borrowing, ownership, move-semantics, references, Box

# Options
A. Allowing data to be mutated through an immutable reference
B. Allowing multiple mutable references to the same data as long as they are not used at the same time

# Hint

# Explanation
The main idea of interior mutability is taking a value of type &T and being able to safely mutate data within T.

Interior mutability means that a value can be mutated even when it is accessed through an immutable reference (`&T`).

Normally in Rust, an immutable reference does not allow mutation. However, some types (like `RefCell`, `Cell`, and `Mutex`) encapsulate their data in a way that allows mutation internally while still presenting an immutable interface.

For example:

```rust
use std::cell::RefCell;

let x = RefCell::new(5);
let r = &x;          // immutable reference
*r.borrow_mut() = 10; // mutation happens through interior mutability
````

Here, `r` is an immutable reference, but we can still mutate the value inside `RefCell`.

This works because these types enforce Rust’s safety rules in other ways (e.g., `RefCell` checks borrowing rules at runtime instead of compile time).

---

Option B is incorrect because interior mutability does not allow multiple mutable references. Rust’s aliasing rules still apply. Instead, types like `RefCell` ensure at runtime that there is only one mutable borrow (or many immutable borrows), preventing unsafe behavior.

