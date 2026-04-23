fn main() {
    // Stack allocated fixed size array
    let stack_array: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
    
    // Heap allocated dynamic vector
    let heap_vector: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0];

    println!("{},{},{}", 
        std::mem::size_of_val(&stack_array), // Array size on stack in bytes
        std::mem::size_of_val(&heap_vector), // Vector struct size in bytes
        heap_vector.capacity() // Vector heap capacity, number of elements
    ); 
    
    // What does this program print?
}