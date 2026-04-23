# Part 1 — Core Invariants of Rust

- **Ownership Uniqueness:** Every value in Rust has exactly one owner at a time; assigning or passing a non-`Copy` value to a new owner *moves* it, invalidating the original binding【21†L27-L36】.  This guarantees that resources (heap data, files, etc.) are freed exactly once.  
  - *Valid:* 
    ```rust
    let a = String::from("hello");
    let b = a;               // `a` is moved into `b`; now `b` owns the string.
    println!("{}", b);       // OK
    ```  
  - *Invalid:* 
    ```rust
    let a = String::from("hello");
    let b = a;
    println!("{}", a);       // Error: use of moved value `a`
    ```  
  - **Enforced because:** Allowing multiple owners of the same resource could lead to double frees or use-after-free bugs【21†L27-L36】.  
  - **Examples:** Ownership comes up in assignments, function arguments, and pattern matching (e.g. destructuring moves fields by default). It underlies *move semantics* and the need for `.clone()` or borrowing to share data.  

- **Aliasing vs. Mutability (“One mutable or many immutable”):** At most one `&mut T` or any number of `&T` references to the same data can exist at once【46†L417-L420】. You cannot have a mutable borrow while an immutable borrow is active.  
  - *Valid:* 
    ```rust
    let mut x = 5;
    let r1 = &x;
    let r2 = &x;            // two immutable borrows – allowed
    println!("{}, {}", r1, r2);
    let r3 = &mut x;        // OK only after r1,r2 go out of scope (after println)
    *r3 += 1;
    ```  
  - *Invalid:* 
    ```rust
    let mut x = 5;
    let r1 = &x;
    let r2 = &mut x;        // ERROR: cannot borrow `x` as mutable because it is already borrowed
    ```  
  - **Enforced because:** This invariant prevents data races and “iterator invalidation” bugs. By forbidding simultaneous mutable and immutable access, Rust guarantees that readers can’t see the data change unexpectedly, and that iterators can’t be invalidated by mutation【46†L282-L289】【48†L85-L89】.  

- **Move Semantics & `Copy` Types:** Non-`Copy` types are *moved* by default, whereas types like integers and tuples of `Copy` types are implicitly copied. A move transfers ownership (making the source unusable) to ensure no dangling ownership【21†L33-L36】.  
  - *Valid with Copy:* 
    ```rust
    let a = 5u32;
    let b = a;              // Copy: a is still usable
    println!("{}", a);      // OK: u32 is Copy
    ```  
  - *Invalid (non-Copy):* 
    ```rust
    let s1 = String::from("foo");
    let s2 = s1;            // s1 is moved into s2
    let s3 = s2;            // s2 is moved into s3
    println!("{}", s2);     // ERROR: value used after move (s2 was moved)
    ```  
  - **Enforced because:** Implicit moving (for non-`Copy` types) prevents multiple pointers to the same heap data. This avoids use-after-free: once ownership is moved, the original binding can’t access freed memory【21†L33-L36】【48†L85-L89】.  

- **Lifetime/Validity of References:** References (`&T`) must always point to valid data; Rust ensures no reference outlives its referent【46†L417-L420】【33†L225-L234】. In other words, you cannot return or store a reference to a value that goes out of scope.  
  - *Valid:* 
    ```rust
    fn no_dangle() -> String {
        let s = String::from("hello");
        s  // moves s out; no dangling
    }
    ```  
  - *Invalid:* 
    ```rust
    fn dangle() -> &String {
        let s = String::from("hello");
        &s   // ERROR: `s` does not live long enough
    }
    ```  
  - **Enforced because:** Allowing a reference to point to freed memory would violate memory safety. The compiler forbids dangling references【46†L319-L327】【33†L225-L233】.  

- **Pattern Matching Exhaustiveness:** A `match` (or other pattern binding) must cover *all possible cases*. The compiler checks that match arms are exhaustive; omitting a variant causes a compile error【23†L27-L30】.  
  - *Valid:* 
    ```rust
    enum E { A, B }
    match E::A {
        E::A => println!("A"),
        E::B => println!("B"),
    }
    ```  
  - *Invalid:* 
    ```rust
    enum E { A, B }
    match E::A {
        E::A => println!("A"),
        // Missing `E::B` or `_` arm → non-exhaustive match error
    }
    ```  
  - **Enforced because:** Exhaustiveness ensures that a `match` can always produce a value and never “falls off”. It prevents runtime surprises by catching missing cases at compile time【23†L27-L30】.  

- **Iterator Consumption:** Iterators are themselves values that are *consumed* when used (e.g. in a `for` loop or when calling `next()`). Once an iterator is moved or exhausted, it cannot be reused.  
  - *Example:*  
    ```rust
    let v = vec![1, 2, 3];
    let mut iter = v.into_iter();  // v is moved into the iterator
    while let Some(x) = iter.next() {
        println!("{}", x);
    }
    // `iter` is now exhausted and can’t be reused. `v` is no longer accessible by name.
    ```  
  - **Appears in:** Moving the collection (e.g. `for x in v`) consumes `v`【37†L242-L251】. Borrowing an iterator (`for x in &v`) lets you use the collection afterward. This invariant follows from ownership rules and is enforced by moves.  

# Part 2 — Failure Modes & Misconceptions

- **Immutable vs. Mutable Borrow Collision:**  
  *Example:*  
  ```rust
  fn main() {
      let mut a = 5;
      let r1 = &a;           // immutable borrow
      let r2 = &mut a;      // ERROR: cannot borrow `a` as mutable because it is also borrowed as immutable
      println!("{}, {}", r1, r2);
  }
  ```  
  *Learner thinks:* “I only have one immutable borrow and then a mutable borrow, and I only print `r1` and `r2` once. It seems sequential, so it should be fine.”  
  *Actually:* The compiler enforces that **no mutable reference can coexist with any active immutable reference**【46†L251-L259】. Here `r1` is still in scope (used later in `println!`), so creating `r2` violates the borrowing invariant【46†L263-L271】.  
  *Fix:* Drop or end the immutable borrow before taking a mutable one. For example, limit the scope of `r1` (e.g. use an inner block or finish printing `r1` first), or simply use `&mut a` only.  
  *Violated invariant:* Aliasing XOR mutability.  

- **Use-After-Move (Value Moved):**  
  *Example:*  
  ```rust
  struct Player { name: String }
  impl Player { fn print(self) { println!("{}", self.name); } }
  fn main() {
      let mut p = Player { name: "Alice".into() };
      p.print();          // p is moved here
      p.print();          // ERROR: use of moved value `p`
  }
  ```  
  *Learner thinks:* “Calling `print()` prints the name. I should be able to call it again.”  
  *Actually:* The method `print(self)` takes ownership of `p`, moving it into the method call【10†L676-L684】. After the first call, `p` has been moved and cannot be used again (Rust prevents a use-after-free scenario).  
  *Fix:* Change the method to borrow instead: `fn print(&self) { ... }`. Then `p` is not moved and can be reused, or clone `p` if you truly need separate ownerships.  
  *Violated invariant:* Ownership uniqueness (move semantics)【21†L33-L36】.  

- **Partial Moves in Patterns:**  
  *Example:*  
  ```rust
  #[derive(Debug)]
  struct Person { name: String, age: u8 }
  fn main() {
      let person = Person { name: "Alice".into(), age: 30 };
      let Person { name, ref age } = person;  // moves `name`, references `age`
      println!("{}", name);
      println!("{}", age);
      println!("{:?}", person);            // ERROR: `person` partially moved
  }
  ```  
  *Learner thinks:* “I moved the `name` out, but I borrowed `age`. I should still be able to use the remaining parts of `person`.”  
  *Actually:* Because `Person` does **not** implement `Copy` (and also has a `Drop` via `String`), destructuring with a move leaves the original `person` only partially valid【5†L25-L33】. Rust forbids using `person` after moving out one of its fields.  
  *Fix:* Do not mix moves and full usage. Either clone before moving, or avoid using `person` again (e.g. continue working with `name` and `age` separately), or use `ref` on both fields.  
  *Violated invariant:* Ownership uniqueness (fields moved invalidate the parent struct) and the partial-move rule【5†L25-L34】.  

- **Iterator Ownership Errors:**  
  *Example:*  
  ```rust
  fn main() {
      let v = vec![1,2,3];
      for x in v { 
          println!("{}", x);
      }
      println!("{:?}", v);  // ERROR: use of moved value `v`
  }
  ```  
  *Learner thinks:* “After the loop, I should still have `v` intact.”  
  *Actually:* Iterating `for x in v` *moves* `v` into the loop【37†L242-L251】. After the loop, `v` is no longer valid.  
  *Fix:* Iterate by reference: `for x in &v { ... }` so `v` is only borrowed and can be reused. Or call `.iter()` explicitly.  
  *Violated invariant:* Iterator consumption / move semantics.  

- **Dangling Reference (Lifetime) Errors:**  
  *Example:*  
  ```rust
  fn make_ref() -> &String {
      let s = String::from("hello");
      &s   // ERROR: returns reference to data that will be dropped
  }
  ```  
  *Learner thinks:* “I’m returning a reference, but `s` is in the same function, maybe Rust will handle it.”  
  *Actually:* `s` is local and is dropped when `make_ref` returns. Returning `&s` would leave a reference to freed memory【33†L225-L233】. The compiler forbids this (lifetime error).  
  *Fix:* Return an owned `String` instead, or take a reference from the caller.  
  *Violated invariant:* References must always be valid (no dangling refs)【33†L225-L233】【46†L417-L420】.  

# Part 3 — Transformation Patterns

We illustrate common refactorings from naive code to Rust-idiomatic code:

- **Cloning → Borrowing:**  
  *Naive:* Cloning data to satisfy ownership. Example:  
  ```rust
  fn print_and_return(input: String) { println!("{}", input); }
  fn main() {
      let s = String::from("hi");
      let s2 = s.clone();  // deep copy to reuse `s`
      print_and_return(s.clone());
      println!("{}", s);   // works, but clones were expensive
  }
  ```  
  *Idiomatic:* Borrow instead of clone. Change functions to take `&String` or `&str`:  
  ```rust
  fn print_and_return(input: &String) { println!("{}", input); }
  fn main() {
      let s = String::from("hi");
      print_and_return(&s);
      println!("{}", s);   // no need to clone
  }
  ```  
  *Reasoning:* Avoid unnecessary heap copies. Borrowing (`&s`) lets `s` stay in place【50†L109-L118】, improving performance and clarity.  

- **Indexing Loops → Iterator Loops:**  
  *Naive:* Using indices to loop over a vector.  
  ```rust
  let mut v = vec![0; 5];
  for i in 0..v.len() {
      v[i] = i * 2;
  }
  ```  
  *Idiomatic:* Use iterator methods:  
  ```rust
  for (i, elem) in v.iter_mut().enumerate() {
      *elem = i * 2;
  }
  ```  
  *Reasoning:* Iterators abstract away manual index management and bounds checks, and often allow simpler, safer code.  

- **Imperative Collection Building → Iterator Chains:**  
  *Naive:* Building a new vector with a `for` loop and `push`.  
  ```rust
  let mut evens = Vec::new();
  for x in 0..10 {
      if x % 2 == 0 {
          evens.push(x);
      }
  }
  ```  
  *Idiomatic:* Use iterator combinators:  
  ```rust
  let evens: Vec<_> = (0..10).filter(|x| x % 2 == 0).collect();
  ```  
  *Reasoning:* Iterator methods (`filter`, `map`, `collect`, etc.) lead to more declarative code. They eliminate mutable state and make the transformation intent clear.  

- **Manual Matching → `if let` / `matches!`:**  
  *Naive:*  
  ```rust
  match opt {
      Some(x) => println!("Value: {}", x),
      None => (),
  }
  ```  
  *Idiomatic:*  
  ```rust
  if let Some(x) = opt {
      println!("Value: {}", x);
  }
  ```  
  *Reasoning:* `if let` is syntactic sugar for the common case of matching one variant and ignoring the rest. It yields shorter code when no action is needed for the other cases.  

- **Direct Loop Mutation → Iterator with `map`:**  
  *Naive:*  
  ```rust
  let mut nums = vec![1,2,3];
  let mut doubles = Vec::new();
  for n in nums {
      doubles.push(n * 2);
  }
  ```  
  *Idiomatic:*  
  ```rust
  let doubles: Vec<_> = nums.into_iter().map(|n| n * 2).collect();
  ```  
  *Reasoning:* Using `.map()` directly expresses the transformation of each element, removing boilerplate.  
  