use std::io;

fn main() {

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    println!("{x}");
    println!("{y}");

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{guess}");

    // Scalar Types

    // Rust has four primary scalar types:
    // 1. integers - signed and unsigned
    // 2. floating-point numbers
    // 3. Booleans
    // 4. characters

    // 1. Integer Types 
    let int_number1: i32 = 1_000; // signed
    let int_number2: u32 = 2_000; // unsigned

    println!("\nInteger Number 1 : {int_number1}");
    println!("nInteger Number 2 : {int_number2}");
    
    // 2. Floating-Point Types
    let float_number1: f32 = 10.55;
    let  float_number2: f64 = 555.10; // default 

    println!("\nFloat_number 1 : {float_number1}");
    println!("Float_number 2 : {float_number2}");

    // 3. Boolean Type
    let is_alive = true;
    let is_raining: bool = false; // with explicit type annotation

    println!("\nIs he alive : {is_alive}");
    println!("Is raining today : {is_raining}");

    // 4. Character Type
    let alphabet = 'A';
    let letter: char = 'X';
    let crab = '🦀';

    println!("\nAlphabet letter : {alphabet}");
    println!("Letter : {letter}");
    println!("Crab Emoji : {crab}");

    //*** Numeric Operations ***//

    // addition 
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.5;

    // multiplication 
    let product = 4 * 30;

    // division 
    let quotient = 56.7 / 35.10;
    let truncated = -10 / 3; // Results in -1

    // reminder 
    let reminder = 43 % 5;

    println!("\n{sum}");
    println!("{difference}");
    println!("{product}");
    println!("{quotient}");
    println!("{truncated}");
    println!("{reminder}");

    //*** END ****/


    // Compound Types

    // Rust has two primitive compound types:
    // 1. tuples 
    // 2. arrays

    // 1. Tuple Type
    let tup = (500, 5.6, 10);
    let (x, y, z) = tup; // destructuring 

    println!("\nThe value of x is {x}");
    println!("The value of x is {y}");
    println!("The value of x is {z}");

    // access a tuple element directly by using a period (.) 
    // followed by the index of the value
    let x: (i32, f64, u8) = (100, 11.4, 30);

    let one_hundred = x.0;
    let eleven_point_four = x.1;
    let thirty = x.2;

    println!("\n{one_hundred}");
    println!("{eleven_point_four}");
    println!("{thirty}");

    // 2. Array Type
    let months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];

    println!("\nThe month is : {}", months[1]);

    // creating an array 
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("The number of array is : {}", a[3]);

    // Accessing Array Elements
    let b = [1, 2, 3, 4, 5];
    let first = b[0];
    let second = b[1];
    let third = b[2];

    println!("\n{first}");
    println!("{second}");
    println!("{third}");

    // Invalid Array Element Access
    let c = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = c[index];

    println!("The value of the element at index {index} is: {element}");
}
