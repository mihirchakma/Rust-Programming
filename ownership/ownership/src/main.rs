fn main() {
    let mut s = String::from("Hello");

    s.push_str(", World"); // push_str() appends a literal to a String

    println!("{s}");

    let str1 = String::from("Hello");

    // Ownership and Functions
    takes_ownership(str1);

    // If we tried to use s after the call to takes_ownership(), 
    // Rust would throw a compile-time error
    // println!("{str1}"); // error

    let x: i32 = 5;

    takes_copy(x);
    println!("{x}");

    let s1 = gives_ownership();
    println!("{s1}");

    let s2 = String::from("Hello");
    // s2 is moved into takes_and_gives_back(), which also moves its return value into s3
    let s3 = takes_and_gives_back(s2);
    println!("{s3}");

    // Return multiple values in Ownership
    let s4 = String::from("Hello");
    let (s5, len) = calculate_length(s4);
    println!("The length of '{s5}' is {len}");

}

// Ownership and Functions
fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn takes_copy(some_integer: i32) {
    println!("{some_integer}");
}

// Return Values and Scope
fn gives_ownership() -> String {

    let some_string = String::from("yours");

    some_string // some_string is returned and moves out to the calling function

}

fn takes_and_gives_back(a_string: String) -> String {

    a_string // a_string is returned and moves out to the calling function
}

// Return multiple values using a tuple
// Returning ownership of parameters
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
