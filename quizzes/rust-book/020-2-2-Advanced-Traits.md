Question: Recall the definition of the Add trait, which of the following best describes why Rhs is a type parameter to the trait Add rather than the function add? That is, why is Add not designed like this:
Type: multiple-choice
Answer: A
Difficulty: 2
Tags: generics

# Options
A. If Rhs were a function-level type parameter, then the definition of add could not assume any structure to Rhs
B. Because trait methods are not allowed to have generic type parameters 

# Hint

# Explanation
Such a design would not make much sense because add<Rhs> would not have any information about the structure of Rhs, so it would be impossible to implement any sensible addition function this way.
