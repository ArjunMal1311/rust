// Vector

fn main(){
    let v = vec![1, 2, 3];
    // let v: Vec<u8> = vec![1, 2, 3];

    println!("{:?}", v);

    let colors = vec!["blue", "red", "green"];
    println!("first color = {}", colors[0]);

    // we can even access it as colors.get(0)


    // We can add values to the vector by making it mutable
    let mut v1 = vec![2, 3, 4, 5, 6];

    v1.push(12);
    println!("{:?}", v1);

    v1.remove(4);
    println!("{:?}", v1);

    for index in 0..3{
        print!("Index: {} Value: {} --", index, v1[index]);
    }

    println!()
    println!()
}