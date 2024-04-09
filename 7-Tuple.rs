fn main() {
    struct Person {
        name: String,
        age: u8,
        height: u8
    }
    
    let person = Person {
        name: String::from("John Doe"),
        age: 18,
        height: 178
    };
    

    println!("Person name = {}", person.name);
    println!("Person age = {}", person.age);
    println!("Person height = {}", person.height);



    // Destructuring Fields of a Rust
    let Person {name, age, height} = person;
    // Simply print it now
}