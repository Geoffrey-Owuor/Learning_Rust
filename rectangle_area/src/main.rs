//  A program to calculate the area of a rectangle
// Using variables first

// Using structs to define a Rectangle struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let height = 30;
    let width = 50;
    println!(
        "The area of the rectangle is {} square pixels",
        area(width, height)
    );

    // Tuple type to hold our required values
    let rect1 = (30, 50);

    println!("The area is {} square pixels", rect_area(rect1));

    // Defining an instance of the rectangle struct
    // This is immutable
    let rect2 = Rectangle {
        width: 50,
        height: 30,
    };

    println!("The struct area is {} square pixels", rect_area2(&rect2));

    // Adding functionality with derived traits
    // How can we print an instance of Rectangle for debugging purposes
    println!("Debugging information for rect2 is {rect2:?}"); //This version is not pretty print
    println!("Pretty print version: {rect2:#?}");

    // Using the dbg! macro
    // Helpful when you really want to figure out what your code is doing
    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale), //takes ownership of expression's value and returns it
        height: 50,
    };

    dbg!(&rect3);
}

// It is not clear anywhere in our program the these two
// Parameters (Height and Width) are related
fn area(height: u32, width: u32) -> u32 {
    width * height
}

fn rect_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn rect_area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
