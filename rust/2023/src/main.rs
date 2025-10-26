mod day1;
mod day2;

use std::io;

use crate::day1::day1;
use crate::day2::day2;

fn main() {
    println!("Advent of Code runner!");

    loop {
        println!("What day do you want to run?");

        print!("> ");

        let mut selection = String::new();

        io::stdin()
            .read_line(&mut selection)
            .expect("Failed to read line");

        match selection.as_str().trim() {
            "day1" => {
                println!("Running day 1");
                day1();
            },
            "day2" => {
                println!("Running day 2");
                day2();
            },
            _ => println!("Unknown selection")
        }
    }
}
