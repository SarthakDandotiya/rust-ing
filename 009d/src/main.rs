// Closures are especially useful when we get to iterators

// Iterators are a pattern that you use to do some task over a sequence of items

// Here is what the iterator trait looks like in Rust
// trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

fn main() {
    let v = vec![1,2,3];

    println!("v {}", v.iter().any(|&x| x!=2));

    for i in v.iter() {
        println!("{}", i);
    }
}
