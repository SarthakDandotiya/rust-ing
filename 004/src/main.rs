// Structs, Methods, Related Functions and the Display/Debug Traits
// We'll see similarithy with other Object Oriented Languages

// Adding Display trait '{}' to our Objects #1
use std::fmt;


#[derive(Debug)]
struct Object {
    width: u32,
    height: u32,
}

// -> is for specifying return and its type
fn area(obj: &Object) -> u32 {
    obj.width * obj.height
    // We don't need to specify return statement but we can if we want to
    // Rust automatically will return the last statement in a function
}

// We can create methods for the object by using the 'impl' keyword creating what is known as the implementation block...Syntax is : impl STRUCT_NAME {}
impl Object { 
    //  We can put in a reference to a keyword called self
    // self will automatically refer to the object that it is tied to and by using the self keyword it defines that particular function as a method
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn show(&self) {
        println!("{}x{} with area of : {}",self.width, self.height, self.area() );
    }

    // We can also create related functions inside of the implementation block
    // Say we want to create a function that creates new objects
    // Related Function
    fn new(width: u32, height: u32) -> Object {
        // Instantiate
        Object {
            width: width, // width field = width variable
            height: height,

            // we can actually just do this:
            // width,
            // height,
            // and this is will still work, since the feild-name and the variable are the same Rust will automatically know that they correspond to each other
        }
    } 
    // This function is not a method
    // A cool thing about the impl block is that it is creating a namespace called Object...so a namespace for the type we are tying it to

} // When we might want to organise things to keep our methods and related functions separate we can create another implementation block so that one impl block only has methods and the other only has related functions
// This wont cause any issue...

// Adding Display trait '{}' to our Objects #2
impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.width, self.height)
    }
}
 
fn main() {
    // Instantiate our object
    let o = Object{
        width: 35,
        height: 55,
    };

    // o.new; this will give us an error
    // to gain acess to the new() function we need to call the namespace
    let obj = Object::new(57, 83); // Now this will create an object with the given values

    // println!("{}x{} with area of : {}",o.width, o.height, o.area() );
    // println!("{}x{} with area of : {}",obj.width, obj.height, obj.area() );
    // ^ Replaced by this...
    o.show();
    obj.show();

    // println!("{:?}", o); We will not be able to print or even debug this...
    // Basically Rust doesnt know how to apply the debug trait to an object we can easily fix this by adding a derive annotation. After doing this we can print out our objects directly as,
    println!("{:#?}", o); // We can even use our pretty debug flags as well
    println!("{:?}", obj); 
    // Still we cant directly print out onjects using the Display trait as we cannot derive it...we need to do it manually
    println!("{}", o);
    println!("{}", obj); 
}