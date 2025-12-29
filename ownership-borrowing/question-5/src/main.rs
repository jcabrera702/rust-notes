fn main() {
    let mut s = String::from("Rust");

    let r1 = &s; //Immutable borrow
    let r2 = &mut s; //Mutable borrow
    println!("{}, {}", r1, r2);
}
