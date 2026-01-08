// A simple if statement

fn main() {
    let number: i32 = 6;

    if number < 5 {
        println!("Condition is true");
    } else {
        println!("Condition is false");
    }

    // Multiple conditions with else if (Only the first true condition is executed)
    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }

    //Using if in a let statement
    // Values that have the potential to be results from each arm of the if statement must be the same type
    let condition: bool = true;
    // As you can see here - we are shadowing the original number variable
    let number: i32 = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
