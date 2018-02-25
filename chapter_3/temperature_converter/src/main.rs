use std::io::{self, Write};

fn main() {
    let units = get_input("Units (c/f): ");
    let units = units.trim();
    let value: f32 = get_input("Value: ").trim().parse().unwrap();

    if units == "c" {
        let farenheit = value * 9.0 / 5.0 + 32.0;
        println!("{}c is {}f", value, farenheit);
    } else if units == "f" {
        let celcius = (value - 32.0) / (9.0 / 5.0);
        println!("{}f is {}c", value, celcius);
    } else {
        println!("Unknown units: {}", units);
    }
}

fn get_input(caption: &'static str) -> String {
    print!("{}", caption);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Could not read");

    input
}
