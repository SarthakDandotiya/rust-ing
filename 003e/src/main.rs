fn main() {
    // no. of times a value is repeating within the vector
    let v = vec![2,4,6,8,4,3,2,6,8,9,4,0,5,6,3];
    for &i in &v {
        let r = count(&v, i);
        println!("{} is printed {} times", i, r);
    }
}

fn count(v: &Vec<i32>, val: i32) -> usize {
    v.into_iter().filter(|&&x| x == val).count()
}