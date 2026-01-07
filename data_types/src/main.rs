fn main() {
    // INTEGERS
    // Converting a string to a numeric
    // Unsigned 32-bit integer
    let guess: u32 = "42".parse().expect("Not a number");
    println!("Guess value: {guess}");

    // FLOATING POINT TYPES
    let x = 2.5; //f64: 64 bit
    let y: f32 = 3.7; //f32: 32 bit

    let sum = x + y;
    println!("Total sum is: {sum}");

    // NUMERIC OPERATIONS
    // Addition
    let sum = 5 + 10;

    // Subtraction
    let difference = 95.5 - 4.3;

    // Multiplication
    let product = 4 * 30;

    // Division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; //Result is -1

    // Remainder - Returns the remainder value of a division
    let remainder = 43 % 5;

    println!(
        "Sum:{sum}, Difference: {difference},
        Product: {product}, Quotient: {quotient},
        Truncated: {truncated}, Remainder: {remainder}"
    );

    // Boolean Types
    // let t = true
    // Character Types
    // let c = 'z';

    // COMPOUND TYPES
    // Tuple type - Tuples have a fixed length - once defined: they cannot grow or shrink in size
    // let tup: (i32, f64, u8) = (500, 6.4, 1)
    // Using pattern matching to destructure a tuple value;
    // let tup = (500, 6.4, 1)
    // let (x, y, z) = tup;
    // println!("The value of y is: {y}");

    // Accessing a tuple element directly using a period (.) followed by the index of the value
    // let x: (i32, f64, u8) = (500, 6.4, 1);
    // let five_hundred = x.0;
    // let six_point_four = x.1;
    // let one = x.2;

    // ARRAY TYPES
    // Every element in the array must have the same type
    // Arrays in rust have a fixed length
    // let a = [1, 2, 3, 4, 5];
    // Array definition with explicit type annotation
    // let a: [i32;5] = [1, 2, 3, 4, 5];
    // An array where each element has the same value
    // let a = [3; 5]; An array of five elements where each element is of value three
    // Accessing elements of an array
    // let a = [1, 2, 3, 4, 5];
    // let first = a[0];
    // let second = a[1];
}
