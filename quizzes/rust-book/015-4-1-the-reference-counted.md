Question: Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
Answer: error
Difficulty: 1
Tags: borrowing, ownership, move-semantics, references, Box

# Hint

# Explanation
Data inside an Rc cannot be mutated without the use of interior mutability (seen next section). Rc enforces this property by implementing the Deref trait, but not implementing the DerefMut trait.
