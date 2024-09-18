fn main() {
    let mut s = String::from("Hello");

    s.push_str(", World"); // push_str() appends a literal to a String

    println!("{s}");
}
