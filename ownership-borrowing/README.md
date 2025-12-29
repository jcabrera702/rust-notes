# Rust Ownership & Borrowing: Job-Ready Guide

This guide covers the fundamental memory management rules in Rust, focusing on the concepts required for professional development.

---

## 1. Ownership & Move Semantics
**Concept:** In Rust, heap-allocated data (like `String`) has a single owner. Assigning the variable to another "moves" ownership, making the original variable invalid to prevent memory bugs like "double-free" errors.

### Question:
Will the following code compile?
```rust
fn main() {
    let s1 = String::from("Hello Rust");
    let s2 = s1;
    println!("The value of s1 is: {}", s1);
}
```
# Rust Ownership & Borrowing: Job-Ready Guide

This guide covers the fundamental memory management rules in Rust, focusing on the concepts required for professional development.

---

## 1. Immutable Borrowing (&)
**Concept:** You can allow other parts of your code to read data without taking ownership by using a reference (`&`). This allows the owner to remain valid for later use.

### Question 2:
How do you make `s2` borrow the value instead of taking ownership?
```rust
fn main() {
    let s1 = String::from("Hello Rust");
    
    // We want s2 to borrow the value, not take ownership
    let s2 = &s1; 

    println!("s1 is: {}, s2 is: {}", s1, s2);
}
```
# Rust Learning: Question 3 - Mutable Borrowing

This section covers how to modify data that is owned by another part of the program using **Mutable References**.

---

## 3. [Mutable Borrowing (`&mut`)](./question-3/src/question.rs)

**Concept:** In Rust, data is immutable by default. To allow a function or another variable to change a value without taking ownership, you must use a **mutable reference** (`&mut`). 

### The Rules for Job-Ready Rust:
1. **Original Mutability:** The variable being borrowed must be declared with `let mut`.
2. **The Borrowing Limit:** You can have exactly **one** mutable reference to a specific piece of data at a time. This prevents "Data Races" where two parts of code try to change the same memory simultaneously.
3. **No Mixing:** You cannot have a mutable reference if any immutable references (`&`) already exist.

---

### Challenge: Fixing the Syntax
To make the following code work, two specific changes were required:

```rust
fn main() {
    // FIX 1: Add 'mut' to the owner so it can be changed
    let mut s1 = String::from("Hello"); 
    
    // FIX 2: Explicitly pass a mutable reference using '&mut'
    change_string(&mut s1); 

    println!("{}", s1); // Prints: Hello, world!
}

fn change_string(some_string: &mut String) {
    some_string.push_str(", world!");
}
```