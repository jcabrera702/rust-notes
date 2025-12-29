fn main() {
    //Will the following code compile? If not, what is the error?
    let s1 = String::from("Hello Rust");
    let s2 = s1;

    println!("The value of s1 is: {}", s1);
   //The compile fails due to s1 being moved and no longer valid when we wrote let s2 = s1;
}
