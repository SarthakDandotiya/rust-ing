// Now lets talk about hash maps
use std::collections::HashMap; // Impoert to gain acess to hashmaps

// The HashMap stors a mapping of keys mapped to a value this is done via hashing function which determines how it places the keys and values in the memory
// A similar thing to this is Distionaries in Python



fn main() {
    let mut hm = HashMap::new();

    // Just like vectors HashMaps also need to be type consistent and cannot accomodate mismatched types in them
    hm.insert(String::from("Random"), 12);
    hm.insert(String::from("String"), 49);
    // we can use the method remove() to not only remove the key but also the value
    // since keys are associated to values, if we just remove the key then that gets rid of the value too
    hm.remove(&String::from("String"));

    for (k, v) in &hm {
        println!("{}: {}", k, v);
    }

    // we can use the get() method to gain access to an element inside of a hashmap
    match hm.get(&String::from("Random")) {
        Some(&n) => println!("{}", n),
        _ => println!("No Match!"),
    }

}
