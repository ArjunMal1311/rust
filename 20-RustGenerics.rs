use std::collections::HashMap;

fn main(){

    // NOT USING GENERICS
    let mut numbers: HashMap<i32, &str> = HashMap::new();

    numbers.insert(1, "One");
    numbers.insert(2, "Two");

    println!("Numbers {:?}", numbers);

    let mut lc: HashMap<&str, &str> = HashMap::new();

    lc.insert("EN", "English");
    lc.insert("HI", "Hindi");

    
    println!("Language {:?}", lc);

    // USING GENERICS

    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }
    
    let int_point = Point { x: 1, y: 2 };
    
    let float_point = Point { x: 1.1, y: 2.2 };
    
    println!("int_point: {:?}", int_point);
    println!("float_point: {:?}", float_point);

}

// Generic Functions

fn my_function<T>(x: T, y: T) -> T {
    // function body
    // do something with `x` and `y`
}

// generic function with multiple generic types
fn my_function<T, U>(x: T, y: U) {
    // function body
    // do something with `x` and `y`
}