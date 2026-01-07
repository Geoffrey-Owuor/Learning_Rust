// Obviously the main function
fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(32, 'h');

    let x = five();
    let y = plus_one(16);
    println!("The value of x is: {x} and y is: {y}");
}

// A custom defined function
fn another_function(x: u32) {
    println!("The value of x is: {x}");
}

// function with more than one parameter
fn print_labeled_measurement(value: u32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Statements and expressions
// statements - instructions that perform some action and do not return a value
// Expressions evaluate to a resultant value

// FUNCTIONS WITH RETURN VALUES
// Functions with return values return the last expression implicitly
// A function that returns a value five
fn five() -> u32 {
    5
}

// Another example
fn plus_one(x: i32) -> i32 {
    x + 1
}
