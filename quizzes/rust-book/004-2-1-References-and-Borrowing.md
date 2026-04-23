Type: text
Question: If you wanted to copy out the number 0 through y, how many dereferences would you need to use? Write your answer as a digit. For example, if the correct expression is *y, then the answer is 1.
Answer: 3
Difficulty: 1
Tags: ownership, move-semantics

# Hint

# Explanation
***y is the correct expression. y has the type Box<&Box<i32>>. It is a heap pointer to a stack reference to a heap pointer. Therefore y must be dereferenced three times, once for each layer of indirection.
