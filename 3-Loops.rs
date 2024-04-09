fn main(){
    // loop {
    //     println!("Hello world");

    //     // break;
    // }

    // This loop will continue to work until we use break statemnt
    // Similarly we have concept of continue


    let mut counter = 1;

    while counter < 6 {
        println!("{}", counter);

        counter += 1;
    }


    // Rust FOR Loop

    for i in 1..6{
        println!("{}", i);
    }
}