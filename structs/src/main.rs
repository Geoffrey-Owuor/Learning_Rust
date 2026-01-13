// Learning about structs
// A custom data type that lets you package together and name multiple related values
// that make up a meaninglful group

// An example of a struct defined globally (Global Struct)
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Creating different types with tuple structs
// Structs that look similar to tuples (tuple structs)
// They don't have the names associated with their fields
// Rather, they have only the type of the fields
// Here are examples of Color and Point structs
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// Defining unit-like structs
// This are structs that don't have any fields
// E.g: struct AlwaysEqual;
// let subject = AlwaysEqual

fn main() {
    // An instance of the struct user
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // Creating a user2 instance in a regular way without the update syntax
    let user2 = User {
        active: user1.active,
        username: String::from("anotherusername"),
        email: String::from("anotherexample.com"),
        sign_in_count: user1.sign_in_count,
    };

    // This is still valid because user1.email was not moved from user1
    user1.email = String::from("anotheremail45@example.com");

    // Assign the struct from the functions
    let user3 = build_user(user1.email, user1.username); //Email and username have been moved here from user1 to user3
    let user4 = build_user_2(user2.email, user2.username); //Email and username have been moved here from user2 to user4

    // Using struct update syntax to achieve the same functionality with lesser code
    let user2 = User {
        email: String::from("anotherexample@gmail.com"),
        username: String::from("the spread username"),
        ..user1
    };

    let user3email = user3.email;
    let user2email = user2.email;
    let user4email = user4.email;

    // Let us print the struct values now
    println!("User 3 email value is {user3email}");
    println!("User 2 email value is {user2email}");
    println!("User 4 email value is {user4email}");

    // Creating instances of the tuple structs
    // let black = Color(0, 0, 0);
    // let origin = Point(0, 0, 0);
}

// A function that returns a User instance with a given email and username
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// Using the field init shorthand
// Where username and email have the same name as struct fields
fn build_user_2(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
