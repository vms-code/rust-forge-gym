Question: What is the maximum number of times a heap allocation could occur in this program? Write your answer in digits, e.g. 0 or 1.
Answer: 7
Type: text
Difficulty: 2
Tags: strings, consumption

# Hint

# Explanation
One allocation for each call to String::from, and one allocation for every time + is called.
