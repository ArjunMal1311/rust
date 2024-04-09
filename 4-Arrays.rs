fn main(){
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];


    // We can similarly ccreate array without data type
    // let numbers = [1, 2, 3, 4, 5];
    println!("Arrays of numbers = {:?}", numbers);



    // Arrays with default values
    let numbers2: [i32; 5] = [3; 5];
    println!("Array of numbers = {:?}", numbers2);


    // Creating a mutable array in Rust and changing the value at certain index

    let mut n = [1, 2, 3, 4, 5];
    n[2] = 66;
    println!("Array of numbers = {:?}", n);


    for index in 0..3{
        println!("Index : {} -- Value: {}", index, n[index]);
    }
    
    let slice = &n[1..3]; // it can be like [..3] (0 to 3(excluding)) or the other way round
    println!("Array of numbers = {:?}", slice);

    // If we want to refernce the whole array simply we do [..]

    // We can create mutable slice in rust
    // let slice = &mut numbers[1..4]
}