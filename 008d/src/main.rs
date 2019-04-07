// We also have another trait called drop
struct A{
    a: String,
}

impl Drop for A {
    fn drop(&mut self) {
        println!("Dropped {}", self.a);
    }
}

fn main() {
    let a = A{a: String::from("A")};
    {
        let b = A{a: String::from("B")};
        {
            let c = A{a: String::from("C")};
            println!("Leaving inner-scope 2");
        }
        println!("Leaving inner-scope 1");
    }
    drop(a);
    println!("Program Ending");
}
