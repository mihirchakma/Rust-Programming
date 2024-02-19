fn main() {

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    println!("{x}");
    println!("{y}");

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{guess}");

    // Scalar Types

    // Rust has four primary scalar types:
    // 1. integers 
    // 2. floating-point numbers
    // 3. Booleans
    // 4. characters

    // Integers 
    let number1: i32 = 1_000;
    let number2: u32 = 2_000;

    println!("\nNumber 1 : {number1}");
    println!("Number 2 : {number2}");
    
    // Floating-Point Types
    
}
