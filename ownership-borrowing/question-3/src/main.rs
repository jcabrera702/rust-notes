fn main() {
    let s1 = String::from("Hello"); // Error 1 is on this line
    
    change_string(__s1); // Error 2: What goes in the blank?

    println!("{}", s1);
}

fn change_string(some_string: &mut String) {
    some_string.push_str(", world!");
}
