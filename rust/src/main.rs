use std::env;
use std::fs;

use aoc;

macro_rules! run_year {
    ($year:ident, $day:expr, $input:expr, [$($day_mod:ident),* $(,)?]) => {
        {
            #[allow(unused_assignments)]
            let mut day_counter = 0u8;
            match $day {
                $(
                    _ if { day_counter += 1; day_counter } == $day => {
                        aoc::$year::$day_mod::solve($input)
                    },
                )*
                _ => {
                    eprintln!("Day {} not implemented for {}", $day, stringify!($year));
                    std::process::exit(1);
                }
            }
        }
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <year> <day>", args[0]);
        eprintln!("Example: {} 2025 1", args[0]);
        std::process::exit(1);
    }

    let year = &args[1];
    let day: u8 = args[2].parse().expect("Day must be a number");

    let input_path = format!("../inputs/year{}/day{:02}.txt", year, day);
    let input = fs::read_to_string(&input_path)
        .unwrap_or_else(|_| panic!("Failed to read input file: {}", input_path));

    match year.as_str() {
        "2025" => run_year!(year2025, day, &input, [day01]),
        _ => {
            eprintln!("Year {} not implemented", year);
            std::process::exit(1);
        }
    }
}
