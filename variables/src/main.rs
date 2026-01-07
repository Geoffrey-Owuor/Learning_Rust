fn main() {
    // Mutable variables
    // We cannot mutate a variables type
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constants - using type annotation
    // Rusts naming convention of constants - using uppercase letters separated by underscores
    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
    println!("Three hours in seconds is: {THREE_HOURS_IN_SECONDS}");

    // Variable Shadowing
    // Shadowing allows us to mutate a variables type
    let y = 12;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");
}
