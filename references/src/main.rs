// Testing code with references and borrowing
// References are immutable by default
fn main() {
    let mut s1 = String::from("hello");

    // Here we are referencing the value in s1 -
    let len = calculate_length(&s1);

    // THE SCOPE OF REFERENCES START FROM WHERE IT IS INTRODUCED AND CONTINUES
    // THROUGH THE LAST TIME THAT REFERENCE IS USED

    // Mutating the value of s1
    // If you have a mutable reference to a value
    // You can have only one mutable reference to that value
    // At a time - This prevents data races
    // Doing something like
    // let r1 = &mut s;
    // let r2 = &mut s; //Here we create another mutable r2 before r1 is even used
    change(&mut s1); //first mutation occurs and finishes

    //Trying to mutate s1 again
    change_two(&mut s1); //second mutation occurs and finishes

    println!("The length of '{s1}' before mutation is {len}.");

    // Check length again
    let len = calculate_length(&s1);

    println!("The length of {s1} after mutation is {len}.");

    //testing the dangling reference
    // let reference_to_nothing = dangle();
}

//The calculate_length function that has a reference to an object as a parameter instead of transferring ownership of a value
// The value of s1 is not mutated here - This only returns the length of the string
fn calculate_length(s: &String) -> usize {
    s.len()
}

// A function for testing mutable references
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn change_two(some_string: &mut String) {
    some_string.push_str(", ahoy");
}

// DANGLING REFERENCES
// A dangling pointer - A pointer that references a location in memory
// that may have been given to someone else - by freeing some memory while
// preserving a pointer to that memory

// Testing how Rust handles dangling references
// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }
