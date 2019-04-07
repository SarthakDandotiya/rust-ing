// We also have an interator trait which is used to implement iterators over collections such as arrays and stuff...

struct Fib {
    c: u32,
    n: u32,
}

impl Iterator for Fib {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let n = self.c + self.n;
        self.c = self.n;
        self.n = n;

        Some(self.c)
    }
}

fn fib() -> Fib {
    Fib{c: 1, n: 1}
}

// We also get various methods we can use on our iterators...

fn main() {
    // this says we want to like take the first 10 elements of our iterator
    for j in fib().take(10) {
        println!("{}", j);
    }

    println!("------------------------", );

    // This skips the first 14 elements, then takes the next 10 in our iterator
    for j in fib().skip(14).take(10) {
        println!("{}", j);
    }

    println!("------------------------", );

    // We also have the next method which allows to iterate over our iterator, one item at a time
    let mut f = fib();
    println!("{:?}", f.next());
    println!("{:?}", f.next());
    println!("{:?}", f.next());
    println!("{:?}", f.next());
    println!("{:?}", f.next());
}
