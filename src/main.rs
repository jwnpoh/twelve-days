use std::io;

fn main() {
    let qn = String::from("How many days of Christmas is your true love gifting you gifts for?");
    println!("{}", &qn);
    // println!("How many days of Christmas is your true love gifting you gifts for?");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    let days: u128 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 1,
    };

    let mut sum: u128 = 0;
    let mut total: u128 = 0;
    let mut index = 0;
    while index < days + 1 {
        sum += index;
        total += sum;

        index += 1;
    }
    println!(
        "The total number of gifts you would receive on the last day is {}.",
        sum
    );

    if days == 1 {
        println!(
            "The sum total number of gifts received over {} day is {}.",
            days, total
        );
    } else {
        println!(
            "The sum total number of gifts received over {} days is {}.",
            days, total
        );
    }
}
