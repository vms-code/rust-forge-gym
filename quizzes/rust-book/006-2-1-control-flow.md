Question: Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
Answer: error
Difficulty: 2
Tags: pattern-matching, borrowing, borrow-checker

# Hint

# Explanation
The match arm Either::Right(s) moves the field s, so x cannot be used in the println.
