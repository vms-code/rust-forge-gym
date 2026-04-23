Type: multiple-choice
Question: The following program does not compile, which of the following best describes the undefined behavior that could occur if this program were allowed to execute?
Answer: B
Difficulty: 1
Tags: ownership, move-semantics

# Options
A. The assignment *n is a use of freed memory
B. There is no undefined behavior in this program

# Hint

# Explanation
This program is safe. No undefined behavior could occur if it were executed. (If i was outside the bounds of v, then Rust will panic at runtime rather than cause undefined behavior.)

The issue is that Rust doesn't know for sure that v[i] and v[i - 1] are referring to different elements.
