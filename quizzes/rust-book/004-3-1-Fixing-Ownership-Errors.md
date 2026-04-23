Type: text
Question: The following code snippet does not compile, which of the following best describes the undefined behavior that could occur if this program were allowed to execute?
Answer: The string is freed twice at the end of the program
Difficulty: 1
Tags: ownership, move-semantics

# Hint

# Explanation
The println is technically safe, since the string won't be deallocated until the end of the current scope. But then undefined behavior occurs, when the string is freed twice on behalf of s and s2.
