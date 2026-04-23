# Part 4 — Skill Decomposition

We break Rust into focused skill units, each targeting one core invariant or related group:

- **Ownership and Moves:**  
  - *Invariant:* Unique ownership, move semantics.  
  - *Typical mistakes:* Using a value after it has been moved; forgetting to return or clone a value.  
  - *Mastery signals:* Can reason about when a move happens (assignment, passing by value, return) and correctly use references or `Clone` to avoid errors.  

- **Immutable Borrowing:**  
  - *Invariant:* You can have many `&T` immutable borrows simultaneously, but they are read-only.  
  - *Mistakes:* Attempting to borrow mutably when immutable borrows exist; misunderstanding that `&T` prevents mutation.  
  - *Mastery:* Writes code with multiple shared borrows without conflicts; knows scopes of `&T` are limited to usage.  

- **Mutable Borrowing:**  
  - *Invariant:* Only one `&mut T` borrow at a time, and no other borrows (immutable or mutable) can coexist.  
  - *Mistakes:* Taking a second mutable borrow; holding an immutable borrow too long when a mutable borrow is needed.  
  - *Mastery:* Uses `&mut` correctly for in-place modification; restructures code (e.g. with scopes) to avoid overlap.  

- **Lifetimes and References:**  
  - *Invariant:* References must not outlive the data they point to; no dangling references.  
  - *Mistakes:* Returning references to local data; storing references longer than source variable’s scope.  
  - *Mastery:* Can reason about borrow scopes and lifetimes; uses owned types or longer-lived references when needed.  

- **Pattern Matching (Irrefutability and Exhaustiveness):**  
  - *Invariant:* `let` patterns must be irrefutable or guarded by `if let`; `match` arms must be exhaustive.  
  - *Mistakes:* Using a non-`Some` pattern in a plain `let`, or forgetting `_` arm in a `match`.  
  - *Mastery:* Covers all cases in `match`; uses `_` or `if let` appropriately; avoids unreachable or missing branches.  

- **Iterator Usage and Consumption:**  
  - *Invariant:* Iterators (and ranges) are lazy and by default move; consuming an iterator (or using `for` loop) may move the underlying collection.  
  - *Mistakes:* Trying to reuse a vector after `for x in v`; expecting a loop to clone elements.  
  - *Mastery:* Chooses between `.iter()`, `.iter_mut()`, and `into_iter()` correctly; can explain when an iterator takes ownership vs borrows.  

- **Cloning vs Borrowing:**  
  - *Invariant:* Cloning creates a deep copy; borrowing avoids allocation.  
  - *Mistakes:* Overusing `.clone()` to appease the compiler; unnecessary cloning of large data.  
  - *Mastery:* Recognizes when a function can take a reference instead; minimizes cloning when possible for efficiency.  

- **Mutable Aliasing (Refs as Read/Write Locks):**  
  - *Invariant:* Borrows behave like locks – immutable borrows are like shared read locks, mutable borrows like exclusive write locks.  
  - *Mistakes:* Treating references like C pointers; assuming multiple `&mut` are okay.  
  - *Mastery:* Applies the “one mutable or many immutable” rule intuitively; avoids simultaneous aliasing that violates it.  

# Part 5 — Exercise System Design

We propose a daily curriculum with four exercise types:

1. **Prediction Exercises (5–10 per day):** Present a short Rust code snippet and ask the student to predict whether it compiles. If it fails, they must identify the error and the violated rule. Each task should isolate one invariant. *Example:* “Does this code compile? If not, why? (Hint: ownership rule)”.  

2. **Fix-It Exercises (5–10 per day):** Give minimally broken Rust code (single error) and ask for a minimal fix. The student must correct the code to compile without altering intended behavior. Each fix should target one concept (e.g. add a borrow, insert a clone, reorder scopes).  

3. **Constraint Transformation Exercises (3–5 per day):** Provide a working code snippet and a constraint (e.g. “refactor without using `.clone()`” or “convert this loop to use iterators”). The student rewrites the code under that constraint. This reinforces idiomatic patterns and invariant-based reasoning.  

4. **Micro-Build Challenges (1–2 per day):** Small real tasks (implement a queue, stack, binary tree, or graph) with added constraints (e.g. “use only immutable references” or “no built-in Vec; use raw pointers safely”). These force composing multiple skills and dealing with edge cases.  

All exercises should *progress in difficulty* and *vary topics*, ensuring no two tasks are just rewording the same problem. Each task must indicate the expected reasoning steps and common pitfalls (to allow solution checking and feedback).

# Part 6 — Exercise Quality Rules

- **Minimal & Focused:** Each exercise isolates one invariant or tight concept. Code should be as short as possible while being non-trivial.  
- **Low Noise:** Remove irrelevant details. Avoid distractions like unrelated syntax or large APIs.  
- **Edge Cases:** Include subtle cases (empty collection, zero-length lifetimes, etc.) to force true understanding.  
- **No Pure Syntax Trivia:** Don’t ask questions that only test memorization of syntax. Emphasize reasoning about ownership, borrowing, and lifetimes.  
- **Explicit Task Wording:** The prompt must clearly state what to predict or fix, without ambiguity.  
- **Solution Uniqueness:** There should be a clear correct solution (up to variable names) so automatic checking is possible.  

# Part 7 — Output: Agent Prompt

```plaintext
You are a Rust exercise generator. Produce a daily set of exercises in JSON format. Each day should include:
- **5–10 Prediction tasks:** For each, provide a Rust code snippet and ask whether it compiles. The answer must include “Yes” or “No”, a brief explanation, the invariant violated if any, and the correct compilation status.
- **5–10 Fix-It tasks:** For each, give broken Rust code (one error) and instructions to fix with minimal changes. The output should include the fixed code, an explanation of the fix, and the violated rule.
- **3–5 Transformation tasks:** For each, give working Rust code and a constraint (e.g. “refactor without cloning”, “use iterators instead of indexing”). Provide the refactored code, reasoning, and mention any invariants reinforced.
- **1–2 Micro-build challenges:** Small implementation tasks (e.g. building a stack, queue, or tree) with constraints to encourage deep reasoning (e.g. only use references, no Vec). Include a problem description, key requirements, expected solution approach, and common mistake to watch out for.

All tasks must target core Rust invariants (ownership, borrowing rules, lifetimes, pattern exhaustiveness, iterator behavior, etc.), with increasing difficulty across the day. Ensure variation: do not repeat the same code patterns. 

Output the result as a JSON object with fields for each exercise type. Each exercise entry should have keys like `type`, `code`, `question`, `answer`, `explanation`, and `common_mistake`. The format must be strict JSON so it can be parsed and evaluated. 

For example, each task object should look like:
```
{
  "type": "prediction" | "fix" | "transform" | "micro",
  "code": "<code snippet here>",
  "question": "<what to do>",
  "answer": "<the correct result or fixed code>",
  "explanation": "<expected reasoning steps>",
  "common_mistake": "<what a learner might do wrong>"
}
```
Make sure answers cite the specific invariant rule and show minimal solutions. 
```

# Part 8 — Example Output

```json
{
  "prediction": [
    {
      "type": "prediction",
      "code": "fn main() {\n    let mut s = String::from(\"hi\");\n    let r1 = &s;\n    let r2 = &mut s;\n    println!(\"{} {}\", r1, r2);\n}",
      "question": "Does this compile? If not, why, and what rule is violated?",
      "answer": "No – it fails to compile.",
      "explanation": "Error: cannot borrow `s` as mutable because it is also borrowed as immutable. The rule violated is Rust’s aliasing rule: you cannot have a mutable borrow (`&mut s`) while an immutable borrow (`&s`) is still in use【46†L251-L259】.",
      "common_mistake": "Thinking that borrowing `s` immutably and then mutably in sequence is allowed; forgetting that `r1` is still in scope when `r2` is created."
    },
    {
      "type": "prediction",
      "code": "fn take(v: Vec<i32>) {}\nfn main() {\n    let v = vec![1,2,3];\n    take(v.clone());\n    println!(\"{:?}\", v);\n}",
      "question": "Will this code compile? If not, why, and what rule is violated?",
      "answer": "Yes – it compiles successfully.",
      "explanation": "The vector `v` is cloned for the call to `take`, so the original `v` is not moved. Using `v` after the call is fine because the clone was moved, not `v` itself【50†L90-L98】.",
      "common_mistake": "Thinking that `take(v.clone())` still moves `v`; actually it moves the cloned copy, so `v` remains valid."
    }
  ],
  "fix": [
    {
      "type": "fix",
      "code": "struct S { x: i32, }\nfn main() {\n    let mut s = S { x: 1 };\n    let r1 = &s;\n    let r2 = &mut s;\n    *r2 = S { x: 2 };\n    println!(\"{}\", r1.x);\n}",
      "question": "Fix the code with minimal changes so it compiles.",
      "answer": "```rust\nstruct S { x: i32, }\nfn main() {\n    let mut s = S { x: 1 };\n    let r1 = &s;\n    // end r1's scope by not using it after this point\n    println!(\"{}\", r1.x);\n    let r2 = &mut s;\n    *r2 = S { x: 2 };\n    println!(\"{}\", r2.x);\n}```",
      "explanation": "The mutable borrow `r2` cannot coexist with the active immutable borrow `r1`. We fix this by ending the use of `r1` (printing and finishing its scope) before creating `r2`, so the borrows don’t overlap【46†L263-L271】.",
      "common_mistake": "Moving the mutable borrow above the println, which still conflicts if `r1` is in scope."
    },
    {
      "type": "fix",
      "code": "fn main() {\n    let s = String::from(\"hello\");\n    let r = &s;\n    drop(s);\n    println!(\"{}\", r);\n}",
      "question": "Fix the code so it compiles with minimal changes.",
      "answer": "```rust\nfn main() {\n    let s = String::from(\"hello\");\n    let r = &s;\n    println!(\"{}\", r);  // use r (and s) before drop\n    drop(s);\n}```",
      "explanation": "We moved the `drop(s)` call after using `r`. Originally `drop(s)` would free `s` while `r` still refers to it, violating the reference validity rule. By printing before dropping, `r` is not dangling【33†L225-L233】.",
      "common_mistake": "Attempting to clone `s` unnecessarily or forgetting that `drop(s)` removes `s`."
    }
  ],
  "transform": [
    {
      "type": "transform",
      "code": "let nums = vec![1,2,3,4];\nlet mut evens = Vec::new();\nfor n in nums {\n    if n % 2 == 0 {\n        evens.push(n);\n    }\n}\nprintln!(\"{:?}\", evens);\n",
      "question": "Refactor this code to use an iterator chain instead of a loop. (No `.clone()` of the vector.)",
      "answer": "```rust\nlet nums = vec![1,2,3,4];\nlet evens: Vec<_> = nums.into_iter().filter(|n| n % 2 == 0).collect();\nprintln!(\"{:?}\", evens);\n```",
      "explanation": "We replaced the manual loop with `into_iter().filter().collect()`. This consumes `nums` (moving it) and builds `evens` in one expression. This is idiomatic and avoids explicit pushes【50†L178-L187】.",
      "common_mistake": "Using `.iter()` and then pushing to `evens` would require referencing and not moving `nums`. The most straightforward is `into_iter().filter().collect()`."
    },
    {
      "type": "transform",
      "code": "let mut data = vec![10, 20, 30];\nlet mut sum = 0;\nfor i in 0..data.len() {\n    sum += data[i];\n}\nprintln!(\"Sum = {}\", sum);\n",
      "question": "Rewrite using `iter()` to avoid indexing.",
      "answer": "```rust\nlet data = vec![10, 20, 30];\nlet sum: i32 = data.iter().sum();\nprintln!(\"Sum = {}\", sum);\n```",
      "explanation": "Using `.iter().sum()` replaces the manual loop. This leverages the Iterator trait and ensures the code still computes the sum. We also removed the mutable borrow of `data`, as `.iter()` borrows it immutably.",
      "common_mistake": "Using `.iter().for_each(|x| sum += x);` requires `sum` to be mutable; using `.sum()` is more concise."
    }
  ],
  "micro": [
    {
      "type": "micro",
      "code": "// Build a stack of integers supporting push, pop, and peek\nstruct Stack { /* your code */ }\n",
      "question": "Implement a simple `Stack<i32>` from scratch without using Vec. Ensure no aliasing/mutable borrow violations. Provide push, pop, and peek methods.",
      "answer": "(A full implementation would be given here, e.g. using a linked list or Box. The solution should respect ownership.)",
      "explanation": "A valid answer might use `Option<Box<Node>>` for the stack links. Students must manage ownership of nodes and ensure only one mutable reference is ever active (e.g. `&mut self` in methods). The challenge highlights dynamic allocation and borrows.",
      "common_mistake": "Using global static or multiple mutable aliases. Another mistake is forgetting to handle the empty stack case in `pop`."
    }
  ]
}
```