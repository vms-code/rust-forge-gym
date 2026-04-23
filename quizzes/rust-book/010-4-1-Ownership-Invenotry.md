Question: If you tried to compile this program, which of the following best describes the compiler error you would get?
Answer: B
Type: multiple-choice
Difficulty: 2
Tags: borrowing, ownership

# Options
A. cannot move out of shared reference in expression &elems[n]
B. cannot borrow elems as mutable for sort

# Hint

# Explanation
The method slice::sort expects a mutable reference to a slice, but instead gets an immutable reference.
