// HashSet implements the set data structure in Rust. Just like a set, it allows us to store values without duplicates.

use std::collections::HashSet;

fn main() {
    let mut colors: HashSet<&str> = HashSet::new();
    
    // insert values in a HashSet
    colors.insert("Red");
    colors.insert("Yellow");
    colors.insert("Green");

    println!("colors = {:?}", colors);

    if colors.contains("Red") {
        println!("We have the color \"Red\" in the HashSet.")
    }

    colors.remove("Yellow");    
    println!("colors after remove operation = {:?}", colors);

    for color in colors {
        // print each value in the hashset
        println!("{}", color);
    }

    // Create HashSet with default set of values using from() method
    let numbers = HashSet::from([2, 7, 8, 10]);
     
    println!("numbers = {:?}", numbers);

    // len()	returns the length of a hashset
    // is_empty()	checks if the hashset is empty
    // clear()	removes all elements from the hashset
    // drain()	returns all the elements as an iterator and clears the hashset

    let hashset1 = HashSet::from([2, 7, 8]);
    let hashset2 = HashSet::from([1, 2, 7]);
    
    // Union of hashsets
    let result: HashSet<_> = hashset1.union(&hashset2).collect();
    // Intersection of hashsets
    let result2: HashSet<_> = hashset1.intersection(&hashset2).collect();
    // Difference between hashsets
    let result3: HashSet<_> = hashset1.difference(&hashset2).collect();
    // Symmetric difference of hashsets
    let result: HashSet<_> = hashset1.symmetric_difference(&hashset2).collect();
    
    
    println!("hashset1 = {:?}", hashset1);
    println!("hashset2 = {:?}", hashset2);
    println!("union = {:?}", result);
    println!("intersection = {:?}", result2);
    println!("difference = {:?}", result3);
    println!("symmetric difference = {:?}", result);
}