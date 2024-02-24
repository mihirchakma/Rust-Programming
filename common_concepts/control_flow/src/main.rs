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
    let number_1 = 9;

    if number_1 % 4 == 0 {
        println!("\nNumber is divisible by 4");
    } else if number_1 % 3 == 0 {
        println!("\nnumber is divisible by 3");
    } else if number_1 % 2 == 0 {
        println!("\nnumber is divisible by 2");
    } else {
        println!("\nnumber is not divisible by 4, 3, or 2");
    }

    // Using if in a let Statement
    let condition = true;
    // let condition = false;

    let number_2 = if condition { 5 } else { 8 };

    println!("\nThe value of number is: {number_2}");



    // Repetition with Loops
    // Rust has three loops:
    // 1. loop
    // 2. while 
    // 3. for 

    // Repeating Code with loop
    loop {
        println!("\nLet's Count!");
        // if we remove break, Program will run forever, 
        // until we stop it manually (ctrl + c)
        break; 
    }

    // Returning Values from Loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("\nThe result is {result}");
}
