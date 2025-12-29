fn main() {
    let reference_to_nothing = dangle();
}
fn dangle() -> &String { //ERROR HERE
    let s = String::from("hello");
    &s
}
//This will fail because the function attempts to return a reference to a variable
//(s) that only exists inside that function's scope

//Why

// In Rust, when a function ends, all of its local variables are "dropped" (deallocated from memory). 
// If Rust allowed the dangle function to return &s, 
// that reference would point to memory that has already been cleaned up. 
// This is known as a Dangling Pointer, 
// and it is a common cause of security vulnerabilities and crashes in languages like C++. 
// Rust's borrow checker prevents this by ensuring that a reference never outlives the data it points to.