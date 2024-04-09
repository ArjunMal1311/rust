// Block Scoped
// so we have inner and outer variables
// When a variable is declared within a particular scope has the same
// name as a variable declared in the outer scope
// This is known as variable shadowing


// Variable Freezing in RUST
// We can freeze a variable in RUST by using shadowing and immutability

fn main() {
    let mut age = 1;
    {
        // shadowing by immutable age variable
        let age = age;

        // error, age variable is frozen in this scope
        age = 2;

        println!("age variable inner block = {}", age);
        // age variable goes out of scope
    }
    // end of the inner block

    // age variable is not frozen in outer block
    age = 3;

    println!("integer variable outer block = {}", age);
}