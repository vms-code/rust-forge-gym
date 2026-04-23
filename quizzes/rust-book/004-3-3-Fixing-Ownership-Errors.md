Question: Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
Answer: 0 2
Difficulty: 2
Warnings: unused
Tags: ownership, move-semantics

# Hint

# Explanation
This program does compile, as the binding of x copies point[0], allowing y to mutably borrow point[1]. The mutation x += 1 does not affect point, while the mutation *y += 1 does, so the final result is 0 2.
