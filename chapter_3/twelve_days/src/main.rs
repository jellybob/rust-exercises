fn main() {
    for n in 1..13 {
        println!("On the {}{} day of Christmas, my true love sent to me,", n, ordinal(n));

        let mut gift_list = String::new();
        let gift_list = gifts(n, gift_list);
        println!("{}", gift_list);
    }
}

fn ordinal(n: u8) -> &'static str {
    match n {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    }
}

fn gifts(n: u8, string: String) -> String {
    let gift = match n {
        12 => "12 drummers drumming",
        11 => "11 pipers piping",
        10 => "10 lords a-leaping",
         9 => "9 ladies dancing",
         8 => "8 maids a-milking",
         7 => "7 swans a-swanning",
         6 => "6 geese a-laying",
         5 => "5 goooooold riiiiiings",
         4 => "4 calling birds",
         3 => "3 french hens",
         2 => "2 turtle doves",
         1 => "a partridge in a pear tree",
         _ => "",
    };

    let new_gifts = String::from(string + gift + "\n");
    if n > 0 {
        return gifts(n - 1, new_gifts);
    }

    return new_gifts;
}
