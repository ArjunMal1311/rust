// https://www.programiz.com/rust/stack-and-heap

// Rust - memory safe - ownership, references and borrowing

// Stack - LIFO
// Data stored on stack must have a fixed size during compile time
// Rust by default allocates memory on the stack for primitive types

// Heap
// Opposed to Stack, most of the time we will pass variables(memory)
// to different functions and keep them alive for longer than 
// a single function's execution

// In head we are basically storing the address and not the value


// Accessing the data is faster in stack than heap
// Managing memory in stack is predictable and trivial whereas in heap it is non trivial
// Rust stack allocates by default and Box is used to allocate the heap

// Primitive types and local variables of a function are allocated on the stack
// Data types that are dynamic in size such as String, Vector, Box etx are allocated on heap 

// Rust Ownership
// We can make the memory live longer by transferring ownership where the heap
// can stay alive than the function which allocates the Box

fn main(){
    let x = Box::new(100);
    ley y = 
};