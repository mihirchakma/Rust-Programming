fn main() {

    // if Expressions
    let number = 5;

    if number < 10 {
        println!("Condition is true");
    } else {
        println!("Condition is false");
    }

    // Rust conditions must be booleans 
    // if there is not bool, it will throws an error
    let num = 2;

    if num != 0 { // if num {}
        println!("\nCondition is true");
    }

    // Handling Multiple Conditions with else if
    let number_1 = 6;

    if number_1 % 4 == 0 {
        println!("\nNumber is divisible by 4");
    } else if number_1 % 3 == 0 {
        println!("\nnumber is divisible by 3");
    } else if number_1 % 2 == 0 {
        println!("\nnumber is divisible by 2");
    } else {
        println!("\nnumber is not divisible by 4, 3, or 2");
    }
}
