//  A program to calculate the area of a rectangle
// Using variables first

// Using structs to define a Rectangle struct
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
