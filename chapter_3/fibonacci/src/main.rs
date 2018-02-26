use std::env;

fn main() {
    if env::args().count() != 2 {
        println!("Usage: fibonacci number");
        std::process::exit(1);
    }

    let number = match env::args().last() {
        Some(x) => x,
        None => String::new(),
    };

    let number: u32 = match number.parse() {
        Ok(x) => x,
        Err(_) => {
            println!("That should be a number");
            0
        },
    };

    println!("fib({}) == {}", number, fib(number));
}

fn fib(n: u32) -> u32 {
    if n < 2 {
        return n
    } else {
        return fib(n-1) + fib(n-2);
    }
}
