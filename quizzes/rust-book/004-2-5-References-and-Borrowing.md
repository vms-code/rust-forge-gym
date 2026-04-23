Type: multiple-choice
Question: Which of the following best describes the point at which undefined behavior occurs in this program?
Answer: A
Difficulty: 1
Tags: ownership, move-semantics

# Options
A. v1[0] reads v1, which points to deallocated memory
B. v1 has its pointer invalidated by the push on line 3

# Hint
undefined behavior: pointer used after its pointee is freed

# Explanation
The undefined behavior arises because v1 is read after being freed. Note that the other three options are correct statements about the program, but they do not explain why the undefined behavior actually occurs. For example, if the println were deleted, then the other three options are still true, but the program no longer has undefined behavior.
