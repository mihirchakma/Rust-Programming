fn main() {
    println!("Hello, Functions!");

    // Calling functions 
    say_hello();
    another_function(10);
    print_date_month_year(21, "February".to_string(), 2024);
}

// Creating Functions 
fn say_hello() {

    let my_name = String::from("Mihir");
    println!("Hello, {my_name}!");
}

fn another_function(x: i32) {
    println!("The value of x is {x}");
}

fn print_date_month_year(date:i32, month: String, year: i64) {
    println!("\nDate: {date} \nMonth: {month} \nYear : {year}.");
}
