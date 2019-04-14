// Closures are anonymous function that you can save into variable and pass as arguments into other functions, you can even have a function that creates a closure 

// General Function in Rust:
// fn f(i: i32) -> i32 { i+1 }

// We can create an anonymous function as:
// let f = |i| i + 1;
// or
// let f = |i: i32| -> i32 i + 1; 
// But the latter one isnt necessary

// Closures are inherently flexible and will do what is required to make it work

fn run <F>(f: F)
where F: Fn() {
    f();
}

fn add3 <F>(f: F) -> i32
where F: Fn(i32) -> i32 {
    f(3)
}

// We can also use closures inside of Structs
struct A<F: Fn(i32) -> i32> {
    f: F
}

fn main() {
    let f = |i| i + 1;
    let x = 10;
    println!("{}", f(x));

    // Without inputs
    let p = || println!("This is a closure");
    p();

    println!("--------------------");

    let mut c = 0;
    let mut inc = || {
        // This will automaticlaly borrow 'c'
        c += 1;
        println!("Incremented by 1: {}", c);
    };

    inc();
    inc();
    inc();

    println!("--------------------");

    let px = || println!("Hello from run Function!");
    run(px);

    let xx = |i| i * 10;

    let a = A{
        f: xx
    };

    println!("3 * 10 = {}", add3(xx)); 
}

// We can also use closures as an output parameter however there is a slight catch to doing this