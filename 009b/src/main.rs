
// Very Simple Example to demonstrate that Box sort of works like Reference does

fn main() {
    let y = 4;
    let x = &y;
    let z = Box::new(y);

    // If pointer of x is equal to pointer of z
    if *x == *z {
        println!("True");
    }
}
