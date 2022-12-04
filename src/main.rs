use std::env;
use std::io::{self, BufRead};

mod day1;
mod day2;
mod day3;
mod day4;

pub fn get_input() -> Vec<String> {
    io::stdin().lock().lines().map(Result::unwrap).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: aoc22 <day>");
        return;
    }

    let day: i32 = args[1].parse().unwrap();

    match day {
        1 => day1::day1(),
        2 => day2::day2(),
        3 => day3::day3(),
        4 => day4::day4(),
        _ => {
            if day < 0 || day > 24 {
                println!("That's not in the advents calendar");
            } else {
                println!("I haven't done that day yet");
            }
        }
    };
}
