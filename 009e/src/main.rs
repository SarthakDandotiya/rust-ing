// Example 

fn is_even(n: u32) -> bool {
    n % 2 == 0
}

fn main() {
    // let top = 10000;
    // let mut c = 0;

    // // In Rust we have lazy iterators which basically means that we might not end up breaking our program if we run into an infinite loop unlike other programming languages
    // // In essence what lazy means is that the compiler will not resolve things unless it actually needs to
    // for n in 0.. {
    //     let x = n * n;

    //     if x >= top {
    //         break;
    //     }
    //     else if is_even(x) {
    //         c += x;
    //     }
    // }
    // println!("{}", c);
    // // Finds the sum of all squared even numbers under 10000

    // // This is the imperative way of doing things

    
    // We can do the above in a much functional way using closure and some of the methods provided with Rust
    let s: u32 = 
    (0..).map(|n| n*n)
    .take_while(|&n| n < 10000)
    .filter(|&n| is_even(n))
    .fold(0, |s, i|  s + i);
    // This is equivalent to the part commented above we just are using higher order functions, closures, etc.

    println!("{}", s);
}
