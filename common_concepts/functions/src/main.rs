fn main() {
    println!("Hello, Functions!");

    // Calling functions 
    say_hello();
    another_function(10);
    print_date_month_year(21, "February".to_string(), 2024);


    // Statements and Expressions
    let a = 6; // statement 

    println!("\n{a}");

    let y = {
        let x = 5; // expression
        x + 5
    }; // note this ; semicolon 

    println!("The value of y is: {y}");


    // Functions with Return Values
    let call_five = five();
    println!("\nThe value of function's return type is : {call_five}");

    let add_one = plus_one(10);

    println!("\nThe value is: {add_one}");

    // print example
    println!("{}", f({
        let y = 1;
        y + 1
      }));
}

// Creating Functions 
fn say_hello() {

    let my_name = String::from("Mihir");
    println!("Hello, {my_name}!");
}

fn another_function(x: i32) {
    println!("The value of x is {x}");
}

// create a function with parameters (arguments)
fn print_date_month_year(date:i32, month: String, year: i64) {
    println!("\nDate: {date} \nMonth: {month} \nYear : {year}.");
}


// Functions with Return Values
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// example 

fn f(x: i32) -> i32 { 
    x + 1 
}
