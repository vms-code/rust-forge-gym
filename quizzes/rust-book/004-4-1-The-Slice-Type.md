Question: Consider the variables s2 and s3 in the following program. These two variables will be located in memory within the stack frame for main. Each variable has a size in memory on the stack, not including the size of pointed data. Which statement is true about the sizes of s2 and s3?
Type: multiple-choice
Answer: A
Difficulty: 2
Tags: ownership, move-semantics

# Options
A. s3 has more bytes than s2
B. s3 has fewer bytes than s2

# Hint

# Explanation
The type &String is a normal reference consisting of a single pointer, so 8 bytes on a 64-bit architecture. The type &str is a special slice reference which consists of a pointer and a length, so 16 bytes. Therefore s3 of type &str uses more memory than s2 of type &String. You can verify this yourself using std::mem::size_of, like so:
```rust
fn main() {
  println!(
    "&String={} &str={}",
    std::mem::size_of::<&String>(),
    std::mem::size_of::<&str>(),
  );
}
```
Also, note that Rust will implicitly convert string references to either &String or &str based on the context of the reference. So the expression &s produces two different values based on the expected type of &s.
