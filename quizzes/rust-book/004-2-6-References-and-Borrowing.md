Question: Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
Answer: error
Difficulty: 1
Tags: ownership, move-semantics

# Hint

# Explanation
Although n is marked as mut, the reference to n must also be marked as mut. So a valid version of this program would say incr(&mut n).
