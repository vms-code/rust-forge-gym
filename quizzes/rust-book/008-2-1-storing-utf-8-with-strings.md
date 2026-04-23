Question: What is the difference between using a + b and a.push_str(b) to concatenate two strings?
Answer: B
Type: multiple-choice
Difficulty: 2
Tags: strings, consumption

# Options
A. + creates a new string while push_str modifies the original string in place
B. + consumes ownership of a, while push_str does not

# Hint

# Explanation
push_str takes &mut self while + takes self, so + consumes ownership and push_str does not.

Note: The `+` operator takes ownership of `a`, but it does not necessarily allocate a completely new buffer. In practice, it often reuses `a`’s existing memory and appends `b` to it.

So while `+` consumes `a` (you can’t use `a` afterward), it may still modify the same underlying buffer rather than creating a brand-new one.

In contrast, `push_str` takes `&mut self` and always modifies the existing string in place without transferring ownership.
