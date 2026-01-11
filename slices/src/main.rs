// slices is a kind of reference that lets you reference a contiguous sequence of elements in a collection
// A function that takes a string of words separated by spaces, and returns
// The first word it finds in that string
fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    let second_word = first_word_vtwo(&s);

    // Using word here after string is cleared
    // Is a bug since s has been cleared
    // They are basically out of sync

    println!("First word index is: {word}");
    println!("First word using second slice function is: {second_word}");

    s.clear(); // Emptying the string to ""
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// Solving the out of sync issue with slices
// An example of using slices
// fn slices_function() {
//     let s = String::from("hello world");

//     let hello = &s[0..5]; //This is same as &s[..5] when starting at index 0
//     let world = &s[6..11]; //This is also same as &s[6..]; since 11 is the length of the string

//     // You can drop both values if you are taking a portion of the entire string
//     // e.g
//     let slice = &s[0..11]; //This will be same as &s[..];
// }

// Re-writing the first_word function using slices
// If we have a String, we can also pass a slice of the String (&str) or a reference to the String
fn first_word_vtwo(s: &String) -> &str {
    //parameter can also be written as (s: &str)
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// String literals are string slices already
// so let word = first_word(&my_string_literal[..])
// Is the same as let word = first_word(my_string_literal)
