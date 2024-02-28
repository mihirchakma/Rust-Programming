use std::io;

// Celsius to Fahrenheit: F = C * (9/5) + 32
fn ctof(celsius: f64) -> f64 {
    (celsius * (9.0 / 5.0)) + 32.0
}

// Fahrenheit to Celsius: C = (F - 32) * (5/9)
fn ftoc(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * (5.0 / 9.0)
}

fn main() {
    println!("Do you want to convert to Celsius or Fahrenheit? Input 'C' or 'F':");

    let mut convert_type = String::new();
    io::stdin()
        .read_line(&mut convert_type)
        .expect("Failed to read conversion type.");

    let t = convert_type.trim().to_lowercase();

    println!("You want to convert to: {}", t);

    println!("What temperature would you like to convert?");
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read temperature.");

    let temp: f64 = match temp.trim().parse() {
        Ok(temp) => temp,
        Err(_) => {
            println!("Invalid temperature input. Exiting.");
            return;
        }
    };

    match t.as_str() {
        "c" => println!("{}°C is approximately {:.2}°F", temp, ctof(temp)),
        "f" => println!("{}°F is approximately {:.2}°C", temp, ftoc(temp)),
        _ => println!("Invalid conversion type. Please input 'C' or 'F'."),
    }
}
