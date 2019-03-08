// Vectors, HashMaps, Casting, If-Let, While-Let, and the Result Enum

#[derive(Debug)]
enum Example {
    Float(f64),
    Int(i32),
    Text(String),
}

fn main() {
    // We have seen vectors before since we had to use them here and there
     let x = vec![1,2,3,4]; 
    //  Now vectors are like resizable arrays and like slices, their size is not known at compile time but they can grow and shring at anytime...
    // A Vector is represented by three pieces of data: a pointer to the data, its length and its capacity
    // the capacity decides how much memory is reserved for the vector and the vector can grow as long as the length is smaller than its capacity, when this is to surpass, the vector is re-allocated with as larger amount og memory
    // An important thing to keep in mind is that vectors can have only one type of vaues in them.
    
    // We can also create a vector this way...
    let mut v = Vec::new(); // empty Vector v created

    v.push(5); // Pushing values into our vector v
    v.push(6);
    v.push(7);
    v.push(8);

    for i in &v { // Printing values in our vector 'v'
        println!("{}", i);
    }
    // Using debug flag and a few other methods of vectors
    println!("{:?} {} {}", &v, v.len(), v.capacity());

    // Now lets add another value to the vector and see what happens to the capacity
    v.push(9);
    println!("{:?} {} {}", &v, v.len(), v.capacity()); // capacity will keep increasing as 4, 8, 16, 32... as we keep adding values that overflow the previous capacity

    // we can also uae another method of pop
    println!("{:?}", v.pop()); // the output will actually give us an option value

    println!("==========");

    // To anotate vectors we put Vect and then we put triangular brackets around the type anotation of the thing you wan to put inside your vector
    let mut w: Vec<i32> = Vec::new();

    // Since w is empty we should get empty vector with 0 length and 0 capacity and then  the answer for the pop method should me None
    for i in &w {
        println!("{}", i);
    }
    println!("{:?} {} {}", &w, w.len(), w.capacity());
    println!("{:?}", w.pop());

    println!("==========");

    // We can embed an enum inside a vector as, because enum is technically a single type dispite the fact that it may have multiple variations and vec only allows single type but doing something like this lets us do much more...
    let r = vec![
        Example::Int(142),
        Example::Float(69.9),
        Example::Text(String::from("This is a String")),
    ];
    println!("{:?}", r);
}
