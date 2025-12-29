fn main() {
    //To prevent data races, Rust only allows one mutable reference to a piece of data at a time
    //If you have a mutable reference, you cannot have any other reference (mutable or immutable)
    //until that mutable reference is no longer being used
    let mut s = String::from("Rust");

    let r1 = &mut s;
    let r2 = &mut s; //Focus here

    println!("{},{}", r1, r2);
}

//This will fail because their are multiple mutable references to s which are r1 and r2
