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

    let s1 = gives_ownership(); //gives ownership moves its value into s1
    let s2 = String::from("hello"); //s2 comes into scope
    let s3 = takes_and_gives_back(s2); //s2 is moved into
    //takes_and_gives_back, which also
    //  moves its return value into s3
    let string_value_original = String::from("hello");
    let (string_value, len) = calculate_length(string_value_original);

    // Note that we cannot access value of s2 here - as it has been moved into s3
    println!("s1 value: {s1}, s2 value: {s3}");
    println!("The length of {string_value} is {len}");
} //Here x, s1, s3 goes out of scope and are dropped. However, because s's and s2's value was moved
//Nothing special happens here

fn takes_ownership(some_string: String) {
    //some_string comes into scope
    println!("{some_string}");
} //Here some_string goes out of scope and drop is called. The backing memory is freed

fn makes_copy(some_integer: i32) {
    //some_integer comes into scope
    println!("{some_integer}");
} //Some integer goes out of scope, nothing special happens here

// Return Values And Scope
fn gives_ownership() -> String {
    //gives_ownership will move its return value
    //to the function that calls it
    let some_string = String::from("yours"); //some_string comes into scope
    some_string //some string is returned and moves out
    //to the calling function
}

// Function that takes a string and gives a string
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope

    a_string //a_string is returned and moves out to the calling function
}

// Returning multiple values using a tuple
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); //len() method returns the length of a string

    (s, length)
}
