use std::io;

// In rust once we declare the variable they are immutable
// let apples = 5;
// Once we give the variables a value, the value wont change

// so we write mut like let mut apples = 5;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}


// Rust Data Types
// 1. Integer
// 2. Floating Point
// 3. Boolean
// 4. Character

// INTEGER
let number: i32 = 200;  // i specififes signer integer, 32 - 32 bits of space in memory
let number2: u32 = 200 // u - unsigned integer, cant stor -ve numbers

// FLOATING
let x: f32 = 3.1;
let y: f64 = 45.000000031;

// RUST BOOLEAN
let flag1: bool = true;

// CHARACTER TYPE
let character: char = 'z';


// Rust automatically identifies data type by looking at the variable
// let x = 51;
// rust automically set it as i32

// Printing
println!("x={}", x)