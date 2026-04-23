Answer: 16,24,4
Difficulty: 1
Tags: ml-fundamentals, arrays, vectors, memory-layout, stack, heap

# Hint

Arrays are fixed size and allocated on the stack. Vec is a struct holding a pointer, length and capacity.

# Explanation

This is the most fundamental distinction you need to internalize for numerical computing in Rust. Every ML and linear algebra library is built on top of these two primitives.

## Question

```rust
fn main() {
    // Stack allocated fixed size array
    let stack_array: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
    
    // Heap allocated dynamic vector
    let heap_vector: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0];

    println!("{}", std::mem::size_of_val(&stack_array)); // Array size on stack in bytes
    println!("{}", std::mem::size_of_val(&heap_vector)); // Vector struct size in bytes
    println!("{}", heap_vector.capacity()); // Vector heap capacity, number of elements
}
```

What does this program print?

## Tags
`ml-fundamentals`, `arrays`, `vectors`, `memory-layout`, `stack`, `heap`

## Difficulty
1

## Explanation

### Arrays `[T; N]`
- Fixed size known at **compile time**
- Allocated directly on the **stack**
- Zero overhead, no pointer indirection
- Exactly `N * size_of::<T>()` bytes in size
- Cannot grow or shrink after creation

For `[f32; 4]`:
`4 elements * 4 bytes = 16 bytes` on the stack, no extra overhead.

### Vectors `Vec<T>`
- Dynamic size known at **runtime**
- Data stored on the **heap**
- The vector struct itself is 3 machine words:
  1. Pointer to heap data (8 bytes on 64-bit)
  2. Length (8 bytes)
  3. Capacity (8 bytes)
- Total struct size: **24 bytes on 64-bit systems**

This is why when you pass a Vec around you are only copying 24 bytes, not the entire data buffer.

## Key Takeaway for ML
✅ Use `[T; N]` for small, fixed size tensors known at compile time (weights, small buffers)
✅ Use `Vec<T>` for dynamic size data, batches, large matrices
✅ Almost all performance optimizations come from understanding this distinction