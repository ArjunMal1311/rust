// Functions without name are closures

// || - start of a closure
// then we have the body of the closure after ||

fn main(){
    let print_text = || println!("Defining Closure");
    print_text();


    let add_one = |x: i32| x + 1;
    println!("{}", add_one(12));


    let squared_sum = |x: i32, y: i32| {
        let sum: i32 = x + y;

        let result: i32 = sum * sum;
        return result;
    };

    let result = squared_sum(5, 3);
    println!("{}", result);
}