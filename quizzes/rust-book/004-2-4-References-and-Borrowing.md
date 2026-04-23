Type: multiple-choice
Question: Which of the following best explains why strs loses and regains write permissions?
Answer: B
Difficulty: 1
Tags: ownership, move-semantics

# Options
A. Because first refers to strs, then strs can only be mutated within a nested scope like the if-statement
B. get_first returns an immutable reference to data within strs, so strs is not writable while first is live

# Hint

# Explanation
When get_first is called, Rust recognizes that the returned string first could point to data within strs, so strs loses write permissions. Once the first variable is no longer used (after the if-condition), then strs regains write permissions.
