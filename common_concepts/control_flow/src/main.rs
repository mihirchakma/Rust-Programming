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



    //*** Repetition with Loops ***
    // Rust has three loops:
    // 1. loop
    // 2. while 
    // 3. for 

    // 1. Repeating Code with loop
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

    println!("\nThe result is {result}\n");

    // Loop Labels to Disambiguate Between Multiple Loops
    let mut count = 0;

    'counting_up: loop {
        println!("Count = {count}");
        let mut remaining = 10;

        loop {
            println!("Remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1
        }

        count += 1
    }
    println!("End count = {count}\n");

    // 2. Conditional Loops with while
    let mut count_number = 5;

    while count_number != 0 {
        println!("{count_number}!");

        count_number -= 1;
    }

    println!("LIFTOFF!!!\n");

    // Looping Through a Collection with for
    let array_list = [10, 20, 30, 40, 50];

    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", array_list[index]);

        index += 1; // index = index + 1;
    }

    println!();
    
    // 3. for loop 
    let arr = [60, 70, 80, 90, 100];

    for element in arr {
        println!("the value is: {element}");
    }

    // to reverse the range:
    for number_item in (1..5).rev() {
        println!("{number_item}!");
    }
    println!("LIFTOFF!!!\n");

    // example 1
    let a = [5; 10]; //  5 repeated 10 times
    let mut sum = 0; // note that the syntax [5; 10] is different from [5, 10]
    for x in a {
        sum += x;
    }
    println!("The sum is: {sum}");

}
