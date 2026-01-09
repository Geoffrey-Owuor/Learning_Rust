// Testing code with references and borrowing
// References are immutable by default
fn main() {
    let mut s1 = String::from("hello");

    // Here we are referencing the value in s1 -
    let len = calculate_length(&s1);

    // Changing the value of s1
    change(&mut s1);

    println!("The length of '{s1}' before mutation is {len}.");

    // Check length again
    let len = calculate_length(&s1);

    println!("The length of {s1} after mutation is {len}.");
}

//The calculate_length function that has a reference to an object as a parameter instead of transferring ownership of a value
fn calculate_length(s: &String) -> usize {
    s.len()
}

// A function for testing mutable references
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
