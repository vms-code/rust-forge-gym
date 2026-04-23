Type: text
Question: Consider the permissions in the following program, at the point marked /* here */, what are the permissions on the path s? Select each permission below, or select "no permissions" if the path has no permissions.
Answer: No permissions
Difficulty: 1
Tags: ownership, move-semantics

# Hint

# Explanation
 The mutable borrow t = &mut s removes all permissions on s while t is live.
