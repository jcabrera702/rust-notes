fn main() {
    //Will the following code compile? If not, what is the error?
    // let s1 = String::from("Hello Rust");
    // let s2 = s1;

    // println!("The value of s1 is: {}", s1);
   //The compile fails due to s1 being moved and no longer valid when we wrote let s2 = s1;
   let s1 = String::from("Hello Rust");

   //We want s2 to borrow the value, not take ownership
   //what goes in the blank?
   let s2 = __s1;
   println!("s1 is: {}, s2 is: {}", s1, s2);
}
