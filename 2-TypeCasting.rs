use std::io;

fn main() {
    // assign a floating point f64 value to decimal variable
    let decimal: f32 = 64.31;

    // convert decimal variable to u16 integer type using as keyword
    let integer = decimal as u16;

    println!("decimal = {}", decimal);
    println!("integer = {}", integer);


    let character: char = 'A';

    // convert char type to u8 integer type
    let integer = char as u8;

    println!("character = {}", character);
    println!("integer = {}", integer);


    // We are allowed to use u8 integers while performing type casting between integer and character

    let integer2: u8 = 65;
    let character2 = integer2 as char;

    println!("character2 = {}", character2);
    println!("integer2 = {}", integer2);



    // Type casting: Boolean to Integer

    let boolean1 = false;
    let integer3 = boolean1 as i32;

    println!(integer3) // 0

    // We cannot convert a floating type to a character!
}