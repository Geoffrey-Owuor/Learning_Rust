// Enums and Pattern Matching
// Using Enums to create custom types
// Enums allow you define a type by enumerating its posssible variants
// I.e, a value is one of a possible set of values
// Fot example. we can say that - Rectangle is one of a set of possible shapes
// that also includes circle and triangle
// An enum value can only be one of its variants

enum ipAddrKind {
    // Attaching associated data to each enum
    V4(String),
    V6(String),
}

enum ipAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Another example of enum
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    //  Instances of the ipAddrKind
    let four = ipAddrKind::V4(String::from("127.0.0.1"));
    let six = ipAddrKind::V6(String::from("::1"));

    // Calling route function with either variant
    // route(four);
    // route(six);

    let home = ipAddr::V4(127, 0, 0, 1);
    let loopback = ipAddr::V6(String::from("::1"));
}

//Defining a function that takes any ipAddrKind
// fn route(ip_kind: ipAddrKind) {}

// Each variant of enum can have different types and amounts
// of associated data
// e.g.
// enum Addr {
// V4(u8, u8, u8, u8),
// V6(String),
// }
