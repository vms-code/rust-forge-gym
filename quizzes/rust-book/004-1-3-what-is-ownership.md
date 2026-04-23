Answer: error
Difficulty: 1
Tags: ownership, move-semantics

# Hint

# Explanation
Because s could be moved inside of the if-statement, it is illegal to use it on line 8. While the if-statement will never execute in this program because b is always false, Rust does not in general try to determine whether if-statements will or won't execute. Rust just assumes that it might be executed, and therefore s might be moved.
