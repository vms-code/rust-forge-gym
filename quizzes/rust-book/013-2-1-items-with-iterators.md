Question: Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
Answer: 4 2
Difficulty: 3
Tags: iterators, collect, functional-style, functions, filter, map, vectors, arrays, iter

# Hint

# Explanation
The order of iterators matters — a filter and a map is not the same as a map and a filter!

You might wonder why the first filter uses *x and the second filter does not. v.iter() produces an Iterator<Item = &i32>. The .filter() call takes an Iterator<Item = T> as input, and passes &T to its predicate. Therefore x: &&i32 on line 3. The Rust standard library implements the remainder operator % for &i32 on the left-hand side (see the docs), but not for &&i32. So we have to dereference x once to use it in the expression *x % 2.

By contrast on line 4, when .map() takes an Iterator<Item = T> as input, it passes T to its closure. Therefore the closure in map takes &i32 as input. The multiplication operator * is implemented for &i32, so x does not need to be dereferenced in x * 2. The operation x * 2 produces a value of type i32, so the result of the map is an Iterator<Item = i32>. The filter then takes x : &i32, which also does not need a dereference to do x % 2. Now you know!
