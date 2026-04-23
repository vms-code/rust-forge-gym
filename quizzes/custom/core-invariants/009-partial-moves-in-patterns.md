Answer: error
Difficulty: 3
Tags: ownership, patterns, partial-move, destructuring

# Hint

Moving out a single field of a struct invalidates the entire struct binding, even if other fields were only borrowed.

# Explanation

This program demonstrates the **Partial Moves in Patterns** failure mode.

```rust
#[derive(Debug)]
struct Person { name: String, age: u8 }

fn main() {
    let person = Person { name: "Alice".into(), age: 30 };
    let Person { name, ref age } = person; // moves `name`, borrows `age`
    println!("{}", name);
    println!("{}", age);
    println!("{:?}", person); // ERROR: `person` partially moved due to use of `person.name`
}
```

The learner might think: *"I moved `name` out, but I only borrowed `age`. The rest of `person` is still there — surely I can still print it."*

In reality, when you move a field out of a struct via pattern destructuring, the parent binding (`person`) becomes **partially moved**. Rust tracks this at the per-field level, but it forbids using the parent value as a whole after any of its fields have been moved out. This is because `person` no longer holds a fully initialized value: the `name` field is gone. Using `person` wholistically (e.g. passing it to a function, printing it with `{:?}`) would be undefined behaviour.

**Why this invariant exists**  
Rust guarantees that `Drop` runs exactly once for the whole value. If you moved a field out and then dropped the parent, the field would be freed twice — or the parent's `Drop` would see uninitialized memory. Disabling use of the partially-moved parent prevents these hazards.

**Fix option 1: avoid using the parent after partial moves — work with the extracted fields directly**
```rust
let Person { name, ref age } = person;
println!("{}", name);
println!("{}", age);
// use `name` and `age` separately — `person` is gone
```

**Fix option 2: borrow all fields (no move)**
```rust
let Person { ref name, ref age } = person;
// or equivalently: let Person { name, age } = &person;
println!("{}", name);
println!("{}", age);
println!("{:?}", person); // OK — nothing was moved
```

**Fix option 3: clone before destructuring**
```rust
let Person { name, .. } = person.clone();
println!("{}", name);
println!("{:?}", person); // OK — original untouched
```
