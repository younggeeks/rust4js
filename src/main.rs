fn main() {
    let age = Box::new(30);
    let umri = 43;
    println!("{:p} => umri {:p}", age, &umri);
}
