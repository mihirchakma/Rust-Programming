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

    // Integers 
    let int_number1: i32 = 1_000; // signed
    let int_number2: u32 = 2_000; // unsigned

    println!("\nInteger Number 1 : {int_number1}");
    println!("nInteger Number 2 : {int_number2}");
    
    // Floating-Point Types
    let float_number1: f32 = 10.55;
    let  float_number2: f64 = 555.10; // default 

    println!("\nFloat_number 1 : {float_number1}");
    println!("Float_number 2 : {float_number2}");

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

    //*** END ****/

}
