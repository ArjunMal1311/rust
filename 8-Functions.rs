fn greet(){
    println!("Hello world!");

    let a = 5;
    let b = 10;

    let sum = a + b;

    println!("{}", sum);
}

fn add(a: i32, b: i32) {
    let sum = a + b;
    
    println!("Sum of a and b = {}", sum);
}

fn subtract(a: i32, b: i32) -> i32 {
    let diff = a - b;
    return diff;
}

fn main(){
    greet();

    add(2, 11);

    // we can also return and shit // let sum = add(2, 11);

    let res = subtract(11, 3);
    println!("{}", res);
}