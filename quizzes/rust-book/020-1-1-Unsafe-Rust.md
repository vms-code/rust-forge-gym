Question: Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
Answer: 0
Difficulty: 1
Tags: unsafe, vectors, for-loop, move-semantics

# Hint

# Explanation
This program is dangerous! It compiles correctly and executes without issue because Vec has enough capacity such that v.push(4) does not resize it. However, if the capacity were 3, then n would point to deallocated memory.
