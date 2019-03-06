fn main() {
    // Flow Control, Conditionals and Pattern Matching

    // First lets look at the conditional operators that we have similar to a lot of other programming Languages
    // ==, !=, <, >, <=, >=, etc.

    // Like a other programming languages Rust also has the 'if' statement
    let n = 6;

    if n < 5 {
        println!("True");
    } else {
        println!("Fasle");
    } // We can get rid of the else clause but then our code will only check for the condition along with if and if that comes out to be true then execute the if block there won't be a case where we check what if the condition along with if isnt true.

    // We can also chain if-else-statements as:
    if n % 4 == 0 {
        println!("n is divisible by 4");
    } else if n % 3 == 0 {
        println!("n is divisible by 3");
    } else if n % 2 == 0 {
        println!("n is divisible by 2");
    } else {
        println!("n is not divisible by 4 or 3 or 2");
    }

    // We can also use if-statements in bindings because if statements are expressions in Rust we can use them in normal bindings
    // Here we are binding the variable 'a' and we are checking if 'c' is true then we bind 50 to a else we bind 70 to it.
    // NOTE: if-statements always need to resolve to a boolean else we will have a problem...
    // Another thing to keep in mind is that if you are going to use this kind of variable binding then the variable type in all cases must be the same...
    let condition = true;
    let a = if condition {
        50
    } else {
        70
    };
    println!("a: {}", a);


    // Lets look at looping stuctures:

    // loop {
    //     println!("Infinite");
    // }
    // This prints out "Infinite" indefinitely

    let mut c = 0;
    loop {
        println!("Finite");
        c += 1 ; // Incrementing c by one

        if c >= 5 {
            break; // This breaks out of the loop once c's value becomes 5 or greater
        }
    }

    // We can also label loops as seen here...
    // The cool thing about labels is that we can actually use our break statement to break a specific loop
    'p: loop {
        println!("loop p");
        'q: loop {
            println!("loop q");
            'r: loop {
                println!("loop r");
                break 'p // Breaking out of the loop labeled q
                // if true {
                //     continue
                // } else {
                //     break
                // }
            }
        }
    }

    // We can also use loop statements as bindings
    let q = loop {
        break 10;
    };
    println!("q: {}", q);


    // Another loop tye in Rust is the 'while' loop
    // While loops work while a conditional is ture and one that condition becomes false, the while loops end
    let mut w = 5;
    while w != 0 {
        println!("{}", w);
        w = w - 1;
    }


    // Finally we have 'for' loops
    let e = vec![10, 20, 30, 40, 50]; // Creating a Vector
    // What will happen is, for a number inside of 'e' we are going to print out 'i' each time we iterate through...so whats going to happen is that we are going to iterate through the vector one item at a time and then this item is going to be bound to 'i' and printed out for each iteration
    for i in e {
        println!("i: {}", i);
    }

    // Another way to use for loops
    // We create a range, '..' is what's called the exclusive range which means it goes from 1 all the way upto 10 and it doesnt include 10
    // We can use '..=' which is an inclusive range, in some cases we need to use '...' though
    for i in 1..10 { // This will print till 9
        println!("{}", i);
    }
    println!("----------");
    for i in 1..=10 { // This will print till 10
        println!("{}", i);
    }
    println!("==========");


    // We discussed about the 'if' statement now lets talk about the 'match' statement
    // 'match' is sort of like 'switch' in a lot of other languages exept its a lot more poweful as its full pattern matching
    let r = 5;
    match r { // This will work exactly like switch
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("something else"), // We need to include this _ case because Rust pattern matching is exhaustive which means that for all integers we need to have atleast one case...here _ is the equivalent to 'default' in 'switch'
    }

    // here is a much more complicated match statement
    let t = 15;
    match t {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("A Prime"),
        13...19 => println!("A Teen"),
        _ => println!("Ain't Special"),
    }

    // here again there is bit more complex patter matching
    let pair1 = (0, -2); // tuple
    match pair1 {
        (0, y) => println!("y: {}", y), // If value on left is zero this runs
        (x, 0) => println!("x: {}", x), // If value on right is zero this runs
        _      => println!("No Match"), // Defaulr case
    }

    // here we have guards as embedded if statements
    // guards ca n be pretty useful if you want to further specify how you are matching your data
    let pair2 = (5, -5);
    match pair2 {
        (x, y) if x == y => println!("Equal"),
        (x, y) if x + y == 0 => println!("Equal Zero"),
        (x, y) if x % 2 == 0 => println!("x is Even"),
        _      => println!("No Match"),
    }

    // Another example of pattern matching
    let p = 5;
    match p {
        // go through the inclusive ranges and match p and bind to n and print
        // the @ allows us to bind the variable to whatever the match is
        // this is very usefull especially when we are doing a match on a vlue that we do not have ownership of...
        n @ 1 ... 12 => println!("n: {}", n),
        n @ 13 ... 19 => println!("n: {}", n),
        _      => println!("No Match"),
    }

    // We can also use match statements as expressions
    let u = 8; 
    let v = match u {
        v @ 1 ... 12 => v,
        v @ 13 ... 19 => v,
        _ => 0,
    };
    // Essentially whats happening here is that it will match u and on finding a match it will bind it to v which will be passed back and bind to variable v...
    println!("v: {}", v);
}
