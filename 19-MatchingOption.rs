fn main() {
    let my_option: Option<i32> = Some(222);
    
    // use of match expression to match Option type
    match my_option {
        Some(value) => println!("The option has a value of {}", value),
        None => println!("The option has no value"),
    }

    let my_result: Result<i32, &str> = Ok(100);

    // use of match expression to match Result type
    match my_result {
        Ok(value) => println!("The result is {}", value),
        Err(error) => println!("The error message is {}", error),
    }

    if let Some(value) = my_option {
        println!("The option has a value of {}", value);
    } else {
        println!("The option has no value");
    }
}