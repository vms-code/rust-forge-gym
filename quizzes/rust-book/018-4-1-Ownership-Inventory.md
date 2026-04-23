Question: If you tried to compile this program, which of the following best describes the compiler error you would get?
Answer: B
Type: multiple-choice
Difficulty: 2
Tags: borrowing, ownership

# Options
A. the type T may be dynamically sized and cannot be casted to a trait object
B. the type T does not live long enough when cast to a trait object

# Hint

# Explanation
When casting Box<T> to Box<dyn Display> (implicitly in v.push(..)), Rust requires that the trait object dyn Display must outlive the vector. However, the lifetime of T is unspecified, so T may not live long enough.
