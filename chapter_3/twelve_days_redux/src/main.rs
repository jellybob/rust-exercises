fn main() {
    let gifts = vec![
        "12 drummers drumming",
        "11 pipers piping",
        "10 lords a-leaping",
        "9 ladies dancing",
        "8 maids a milking",
        "7 swans a swimming",
        "6 geese a laying",
        "5 gooooold riiiings",
        "4 calling birds",
        "3 french hens",
        "2 turtle doves",
        "a partridge in a pear tree",
    ];

    for n in 1..13 {
        println!(
            "On the {}{} day of Christmas, my true love sent to me,",
            n, ordinal(n)
        );

        if n == 1 {
            println!("{}", &gifts[11]);
        } else {
            for gift in &gifts[12-n..11] {
                println!("{},", gift);
            }
            println!("And {}", &gifts[11]);
        }

        println!("");
    }
}

fn ordinal(n: usize) -> &'static str {
    match n {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    }
}
