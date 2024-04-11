fn main() {
    
    fn min<T: PartialOrd>(a: T, b: T) -> T {
        if a < b {
            return a;
        } else {
            return b;
        }
    }

    let result1 = min(2, 7);    
    let result2 = min(2.1, 1.1);
    
    println!("Result1 = {}", result1);
    println!("Result2 = {}", result2);
}