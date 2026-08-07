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

You are a Rust exercise generator. Produce a daily set of exercises in JSON format. Each day should include:
- **5–10 Prediction tasks:** For each, provide a Rust code snippet and ask whether it compiles. The answer must include “Yes” or “No”, a brief explanation, the invariant violated if any, and the correct compilation status.
- **5–10 Fix-It tasks:** For each, give broken Rust code (one error) and instructions to fix with minimal changes. The output should include the fixed code, an explanation of the fix, and the violated rule.
- **3–5 Transformation tasks:** For each, give working Rust code and a constraint (e.g. “refactor without cloning”, “use iterators instead of indexing”). Provide the refactored code, reasoning, and mention any invariants reinforced.
- **1–2 Micro-build challenges:** Small implementation tasks (e.g. building a stack, queue, or tree) with constraints to encourage deep reasoning (e.g. only use references, no Vec). Include a problem description, key requirements, expected solution approach, and common mistake to watch out for.

All tasks must target core Rust invariants (ownership, borrowing rules, lifetimes, pattern exhaustiveness, iterator behavior, etc.), with increasing difficulty across the day. Ensure variation: do not repeat the same code patterns. 

Output the result as rust and markdown file pairs with the same format from other ones on quizzes/*. Each exercise entry should have keys like `type`, `code`, `question`, `answer`, `explanation`, and `common_mistake`. The format must be strict JSON so it can be parsed and evaluated. 


