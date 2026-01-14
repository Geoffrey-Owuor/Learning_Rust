// Methods are a bit similar to functions
// They are defined with the fn keyword
// And they can return a value and have parameters
// The difference - they are defined within a context of a struct,
// Or an enum, or trait object
// Their first parameter is always self - which is the instance
// Of the struct the method is being called on

#[derive(Debug)] //This allows us to view debugging info for the struct type
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Functions defined here (within an impl block) are associated functions
    // Good to use borrowing in methods - Ownership of the struct instance
    // Remains in the main function during method call
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method that returns true/false if the rectangle's width
    // Is greater than zero
    fn width(&self) -> bool {
        self.width > 0
    }

    // Methods with more parameters check if rect1 instance
    // can hold another rect instance of Rectangle
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // We can define functions that do not have a self as their
    // First parameter - and thus are not methods
    // Because they do not need an instance of the type to work with
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a non-zero width; it is {}", rect1.width);
    }

    // Check if rect1 can hold either rect2 or rect3
    // Using a can_hold method
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Associated Functions
    // Calling an associated function to define square dimensions from the Rectangle struct
    let sq = Rectangle::square(3);
    // Printing debugging info from sq instance
    println!("Square dimensions are: {sq:#?}");
}
