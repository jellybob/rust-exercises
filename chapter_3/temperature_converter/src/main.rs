use std::io::{self, Write};

fn main() {
    let unit = get_input("Units (c/f): ");
    let unit = unit.trim();

    let value: f32 = get_input("Value: ").trim().parse().unwrap();

    match unit {
        "c" => {
            let farenheit = value * 9.0 / 5.0 + 32.0;
            println!("{}c is {}f", value, farenheit);
        },
        "f" => {
            let celcius = (value - 32.0) / (9.0 / 5.0);
            println!("{}f is {}c", value, celcius);
        },
        _ => println!("Invalid unit {}", unit),
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
