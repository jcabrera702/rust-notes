fn main() {
    let mut s = String::from("Rust");

    let r1 = &s; //Immutable borrow
    let r2 = &mut s; //Mutable borrow
    println!("{}, {}", r1, r2);
}

// immutable + mutable
// -prevents data from being changed while someone else is reading it
// -you cannot have a mutable reference while any immutable references
//  are still active or in use
// -this rule exists to ensure data integrity
// -the data could suddenly change without warning
// -by blocking this, Rust guarantees that as long as you have a read-
//  only view of data, that data will remain constant
