// Derive annotation
// Allows us to print out an instantiation on A with a debug flag so that we can see it in the console
// We also have the copy and clone traits
// Clone is interesting as it gives a little bit of control over how our variables work in our ownership system
// If we want to make it so that we dont have to reference clone specifically every single time, we can use the 'Copy' trait... it basically means that everytime a function or something borrows our data, it will automatically get copied
#[derive(Debug, Clone, Copy)]
struct A(i32);

// We also have multiple comparison traits aswell...
// Eq -> Trait for equality comparisons which are equivalence relations
// PartialEq -> Trait for equality comparisons which are partial equivalence relations
// PartialOrd -> gives us anti-symmetry, transivity and others
// Ord -> Gives us asymmetry and transitive operations as well
struct B(f32);

fn main() {
    let a = A(32);
    let b = B(12.13);
    let c = a.clone(); // Method to clone when the clone trait is available
    // Now instead of creating an issue 'a' will still reference to the data and 'c' also exists as its having a clone of the data
    let d = a;
    println!("{:?}", a); // {:?} -> Way to print using the debug trait
}
