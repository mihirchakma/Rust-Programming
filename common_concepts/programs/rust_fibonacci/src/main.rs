use std::io;

fn main() {

    println!("To quite the program type -> `exit` ");

    loop {
        println!("Enter a positive number: ");
        let mut int_number = String::new();
        io::stdin().read_line(&mut int_number).expect("Failed to read your input");
        
        if int_number.trim() == "exit" {
            break;
        }
        let int_num: u32 = match int_number.trim().parse() {
            Ok(int_num) => int_num,
            Err(_) => continue,
        };

        println!("Fibonacci ({}) => {}", int_num , fib(int_num));
    }
}

fn fib(n: u32) -> u32 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fib(n-1) + fib(n-2);
    }
}