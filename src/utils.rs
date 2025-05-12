use std::io;

pub fn input(input_text: &str) -> String {
    println!("{}", input_text);
    let stdin = io::stdin();
    let mut name = String::new();
    stdin.read_line(&mut name).expect("Error getting the input");
    name.trim().to_string()
}
