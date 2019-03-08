// If-Let & While-Let

fn main() {

    // Consider this statement here, we have this value s which  has Some and the a character inside of it
    // Now if we want to get to that char we have to write match block
    // This can be a bit unwieldy and thats why we have the 'if let' binding
    let s = Some('c');

    match s {
        Some(i) => println!("{}", i),
        _ => {},
    }

    // Kinda like destructuring...
    if let Some(i) = s {
        println!("{}", i);
    } /* else {
        {}
    } */ // optional block

    // On looking at the output, we can verify that both do the same thing...
    // Now the only catch to using an if let binding is that its not exhaustive like a match binding

    // An even more verbose example
    let mut s = Some(0);

    loop {
        match s {
            Some(i) => if i > 19 {
                println!("Quit");
                s = None;
            } else {
                println!("{}", i);
                s = Some(i + 2);
            },
            _ => {
                break;
            }
        }
    }

    println!("----------");
    // Now again instead of writing all that, we can use 'while let'

    let mut s = Some(0);
    
    while let Some(i) = s {
        if i > 19 {
            println!("Quit");
            s = None;
        } else {
            println!("{}", i);
            s = Some(i + 2); 
        }
    } // Also does the same thing as the previous block did...

    // Again 'while let' is not exhaustive like the part that implements the same functionality without it...
}
