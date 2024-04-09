// The Rust HashMap data structure allows us to store data in key-value pairs. Here are some of the features of hashmap:

// Each value is associated with a corresponding key.
// Keys are unique, whereas values can duplicate.
// Values can be accessed using their corresponding keys.

use std::collections::HashMap;

fn main() {
    let mut fruits: HashMap<i32, String> = HashMap::new();
    
    // add key-value in a hashmap
    fruits.insert(1, String::from("Apple"));
    fruits.insert(2, String::from("Banana"));
    
    println!("fruits = {:?}", fruits);

    let first_fruit = fruits.get(&1);
    let second_fruit = fruits.get(&2);
    let third_fruit = fruits.get(&3);
        
    println!("first fruit = {:?}", first_fruit);
    println!("second fruit = {:?}", second_fruit);
    println!("third fruit = {:?}", third_fruit);

    fruits.remove(&1);
    
    // update the value of the element with key 1
    fruits.insert(1, String::from("Mango"));

    // Other Methods of Rust HashMap
    // Besides the basic methods, here are some more commonly used HashMap methods.

    // Method	Description
    // len()	Returns the length of the HashMap.
    // contains_key()	Checks if a value exists for the specified key.
    // iter()	Returns an iterator over the entries of a HashMap.
    // values()	Returns an iterator over the values of a HashMap.
    // keys()	Returns an iterator over the keys of a HashMap.
    // clone()	Creates and returns a copy of the HashMap.
}