Question: Which of the following best describes why Output is an associated type, while Rhs is a type parameter?
Type: multiple-choice
Answer: A
Difficulty: 2
Tags: generics

# Options
A. A type T should be addable to many other types S, but a given T + S operation should always have a single output type
B. Because associated types are faster at runtime than generic type parameters

# Hint

# Explanation
It is true that an associated type cannot currently have a default, but that is not the main motivating factor for having Rhs be a type parameter in the Add trait.
