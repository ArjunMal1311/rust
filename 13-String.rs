fn main(){
    // let word = String::from("Hello world");
    let mut word = String::from("cat");
    println!("{}", word);

    word.push_str(" dog");
    println!("changed string = {}", word);

    let slice = &word[0..5];
    println!("string = {}", word);
    println!("slice = {}", slice);


    for char in word.chars() {
        println!("{}", char);
    }
    
    // create an empty string
    let mut word = String::new();
}