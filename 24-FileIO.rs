use std::fs::File;

fn main() {
    // Open a file in read only mode in the local file system
    let data_result = File::open("data.txt");

    // Reading a file returns a Result enum
    // Result can be a file or an error
    let data_file = match data_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the data file: {:?}", error),
    };

    println!("Data file: {:?}", data_file);

    let mut file_content = String::new();

    // Reading a file in RUST
    // Copy contents of file to a mutable string
    data_file.read_to_string(&mut file_content).unwrap();

    println!("File content: {:?}", file_content);


    // Writing to a file
    let mut data_file = File::create("data.txt").expect("creation failed");

    // Write contents to the file
    data_file.write("Hello, World!".as_bytes()).expect("write failed");

    println!("Created a file data.txt");


    // Remove a file in RUST
    fs::remove_file("data.txt").expect("could not remove file");
    
    println!("Removed file data.txt");


    // APPENDING file in Rust
    let mut data_file = OpenOptions::new()
        .append(true)
        .open("data.txt")
        .expect("cannot open file");

    // Write to a file
    data_file
        .write("I am learning Rust!".as_bytes())
        .expect("write failed");

    println!("Appended content to a file");
}