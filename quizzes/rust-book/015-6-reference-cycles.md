Question: Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
Answer: 3 1
Difficulty: 1
Warnings: unused
Tags: borrowing, ownership, move-semantics, references, Box

# Hint

# Explanation
The three strong refs are r1, r5, and r6. The one weak ref is r4, which is dropped at the end of main. r2 is dropped at the end of its scope.
