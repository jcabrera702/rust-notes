fn main(){
    let mut s1 = String::from("Hello");
    change_string(&mut s1);
    println!("{}", s1);
}
fn change_string(some_string: &mut String){
    some_string.push_str(", world!");
}