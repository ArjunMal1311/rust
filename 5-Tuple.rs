fn main(){
    let tuple = ("Hello", 5, 3.14); // Eg of tuple without datatype
    // let tuple: (&str, u8, f32) = ("Hello", 5, 3.14); // Tuple with datatype

    println!("{:?}", tuple);


    // Accessing elements of tuple
    // print -- tuple.0    [index printing starting from 0]


    // we can have mutable tupple as well
    let mut tup = ("Hello", 1, 3);
    tup.2 = 69;

    // Destructuring the tuple

    let (name, age, bp) = tup;
    println!("{} {} {}", name, age, bp);
}