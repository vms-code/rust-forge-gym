Question: Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
Answer: error
Difficulty: 2
Tags: ergonomics, functions, functional-style

# Hint

# Explanation
The call to is_equal passes values of type &str. That means T = str. However, str is only allowed to be used in such a generic function if T is marked as ?Sized.

the compiler error message is:
```rust
error[E0277]: the size for values of type `str` cannot be known at compilation time
 --> src\main.rs:5:20
  |
5 |     println!("{}", is_equal("Hello", "world"));
  |                    ^^^^^^^^ doesn't have a size known at compile-time
  |
  = help: the trait `Sized` is not implemented for `str`
note: required by an implicit `Sized` bound in `is_equal`
 --> src\main.rs:1:13
  |
1 | fn is_equal<T: Eq>(t1: &T, t2: &T) -> bool {
  |             ^ required by the implicit `Sized` requirement on this type parameter in `is_equal`
help: consider relaxing the implicit `Sized` restriction
  |
1 | fn is_equal<T: Eq + ?Sized>(t1: &T, t2: &T) -> bool {
  |                   ++++++++

For more information about this error, try `rustc --explain E0277`.
error: could not compile `temporary` (bin "temporary") due to 1 previous error
```

