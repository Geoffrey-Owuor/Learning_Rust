// Testing code with references and borrowing
fn main() {
    let s1 = String::from("hello");

    // Here we are referencing the value in s1
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

//The calculate_length function that has a reference to an object as a parameter instead of transferring ownership of a value
fn calculate_length(s: &String) -> usize {
    s.len()
}
