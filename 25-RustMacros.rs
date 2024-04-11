// Rust Macros
// A macro is a pieve of code that generates another pieve of code

macro_rules! macro_name {
    (...) => {...}
}

// A simple macro named `hello_world`
macro_rules! hello_world {
    () => {
        println!("Hello, World!")
    };
}

macro_rules! print_message {
    // Match rule that takes an argument expression
    ($message:expr) => {
        println!("{}", $message)
    };
}

// A macro which uses repetitions
macro_rules! repeat_print {
    // match rule which matches multiple expressions in an argument
    ($($x:expr),*) => {
        $(
            println!("{}", $x);
        )*
    };
}


fn main() {
    hello_world!();
    print_message!("I am learning Rust!");
    repeat_print!(1, 2, 3);
}

// There are many designators that we can use inside a macro rule body:

// stmt: a statement
// pat: a pattern
// expr: an expression
// ty: a type
// ident: an identifier
// …