Answer: error
Difficulty: 2
Tags: ownership, move-semantics, methods

# Hint

A method that takes `self` (by value) moves the receiver — the caller can no longer use it afterward.

# Explanation

This program demonstrates the **Use-After-Move** failure mode.

```rust
struct Player { name: String }

impl Player {
    fn print(self) {              // takes ownership of self
        println!("{}", self.name);
    }
}

fn main() {
    let p = Player { name: "Alice".into() };
    p.print(); // p is moved into print()
    p.print(); // ERROR: use of moved value `p`
}
```

The learner might think: *"Calling `print()` just prints the name — I should be able to call it again."*

However, `fn print(self)` takes `self` **by value**, which means it **takes ownership** of `p`. After the first call, `p` has been moved into the method and is no longer accessible in `main`. The compiler catches this and reports a "use of moved value" error.

**Why this invariant exists**  
Rust's ownership system guarantees that every value has exactly one owner. When you pass a value to a function (or method) that takes ownership, the caller is no longer the owner. This prevents use-after-free bugs: the value's memory will be freed when the method returns, and accessing `p` in `main` afterward would be undefined behaviour in a language without these guarantees.

**Fix option 1: borrow instead of consuming**
```rust
impl Player {
    fn print(&self) {             // borrows self — caller retains ownership
        println!("{}", self.name);
    }
}

fn main() {
    let p = Player { name: "Alice".into() };
    p.print(); // borrows p
    p.print(); // borrows p again — both calls are valid
}
```

**Fix option 2: clone before calls if you need separate owned copies**
```rust
p.clone().print();
p.print();
```

The idiomatic choice is almost always to use `&self` for read-only methods so the caller retains ownership.
