Question: Say you are writing a function with the following spec:

find_contains takes as input a collection of strings and a target substring. It returns a list of all the strings in the collection that contain the target substring.

Which of the following is the most appropriate type signature for a function implementing this spec?
Type: multiple-choice
Answer: B
Difficulty: 2
Tags: ownership, move-semantics

# Options
A. fn find_contains(haystack: &[String], needle: &str) -> Vec<String>;
B. fn find_contains(haystack: &[String], needle: &str) -> Vec<&String>;

# Hint

# Explanation
For haystack, the slice type &[String] can accept more inputs than &Vec<String>, so it is preferred. For needle, the target substring does not need to be heap-allocated, so &str is preferred to String. For the return type, Vec<String> is not desirable because it would require cloning the input strings. &[String] is not desirable because it can only return a contiguous subsequence of the input. Vec<&String> is the most preferable because it only incurs the cost of allocating the vector, not the strings themselves.
