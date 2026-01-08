fn main() {
    // Returning values from loops
    // let mut counter: i32 = 0;

    // let result: i32 = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is {result}");

    // Loop with labels - Here we label the outer loop
    // let mut count: i32 = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining: i32 = 10;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;
    // }
    // println!("End count = {count}");

    // Using a while loop

    // let mut number: i32 = 3;

    // while number != 0 {
    //     println!("{number}");
    //     number -= 1;
    // }

    // println!("LIFTOFF!!!");

    // Looping through an array with a while loop

    // let a: [i32; 5] = [10, 20, 30, 40, 50];
    // let mut index: usize = 0;

    // while index < 5 {
    //     println!("The value is: {}", a[index]);

    //     index += 1;
    // }

    // Looping through an array with a for loop
    // let a: [i32; 5] = [10, 20, 30, 40, 50];

    // for element in a {
    //     println!("The value is: {element}");
    // }

    // A simple for loop with range
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}
