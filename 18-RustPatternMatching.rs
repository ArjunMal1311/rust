fn main(){
    let x = 2;

    match x {
        1 => println!("x is 1"),
        2 => println!("x is 2"),
        _ => println!("x is something else")
    }

    enum Color {
        Red,
        Green,
        Blue,
    }

    let my_color = Color::Green;

    // use of match expression to match against an enum variant
    match my_color {
        Color::Red => println!("The color is red"),
        Color::Green => println!("The color is green"),
        Color::Blue => println!("The color is blue"),
    }
}