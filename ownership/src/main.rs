// Testing the concept of ownership in Rust
// Here we cannot call s1 in println since its value has been moved to s2
// Hence it is gone out of scope
fn main() {
    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{s2}, world");

    // Scope and assignment when assigning a new value
    // Here rust analyzer shows us a warning since the original value of s is never read since
    // It has now been overwritten
    // let mut s = String::from("Hello");
    // s = String::from("ahoy");

    // println!("{s}, world!");

    // Variables and data interacting with Clone
    // Here use the clone method to deeply copy the stringv value in the heap (Duplicate)
    // The heap data gets copied instead of only copying the stack data containing the pointer to the actual data

    // let s11 = String::from("Hello");
    // let s22 = s11.clone();

    // println!("s11 = {s11}, s22 = {s22}");

    // // An example of stack only data copy
    // // Here, both x and y are still valid
    // let x = 5;
    // let y = x;

    // println!("x = {x}, y = {y}");

    // Ownership and Functions
    let s = String::from("hello"); //s comes into scope

    takes_ownership(s); //value of s moves into the function - hence is no longer valid here

    let x = 5; //x comes into scope

    makes_copy(x); //i32 implements the copy trait
    //x does not move into the function
    //so it is ok to use x afterward
} //Here x goes out of scope, then s. However, because s's value was moved
//Nothing special happens here

fn takes_ownership(some_string: String) {
    //some_string comes into scope
    println!("{some_string}");
} //Here some_string goes out of scope and drop is called. The backing memory is freed

fn makes_copy(some_integer: i32) {
    //some_integer comes into scope
    println!("{some_integer}");
} //Some integer goes out of scope, nothing special happens here
