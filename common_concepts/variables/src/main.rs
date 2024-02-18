fn main() {

    // In Rust, Variables are immutable by default
    // to change/mutable the variable's value, we use mut 
    let mut x = 5;
    println!("The value of x is {x}");

    x = 10;
    println!("The value of x is {x}");

    // CONSTANTS 
    const NIC:u32 = 19971010;
    println!("Your NIC is {NIC}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("HOURS_IN_SECONDS: {THREE_HOURS_IN_SECONDS}");

    // Shadowing 
    // we can declare a new variable with the same name as a previous variable.
    // Rustaceans say that the first variable is shadowed by the second.
    // which means that the second variable is what 
    // the compiler will see when you use the name of the variable.
    let number = 10;
    let number = number + 2;

    {
        let number = number * 5;
        println!("The value of number in the inner scope is: {number}");
    }

    println!("The value of number is: {number}");

    let spaces_str = "     ";
    let spaces_num = spaces_str.len();

    println!("There are {spaces_num} spaces.");

}
