pub mod day_1;
pub mod day_2;
pub mod day_result;

use crate::input::*;
use crate::days::day_1::day_one;
use crate::days::day_2::day_two;
// use crate::Modes;
use crate::day_result::DayResult;


/// Run one specific day
///
/// # Arguments
///
/// * `day` - The index of the name starting by 1
/// * `input` - The [Challenge](crate::input::Challenge) which holds the input
///
/// # Examples
///
pub fn run_day(day: u8, input: Challenge) -> DayResult {
 
    let result = match day {
        1 => {day_one(input)},
        2 => {day_two(input)},
        // 3 => println!("Challenge for day 3"),
        // 4 => println!("Challenge for day 4"),
        // 5 => println!("Challenge for day 5"),
        // 6 => println!("Challenge for day 6"),
        // 7 => println!("Challenge for day 7"),
        // 8 => println!("Challenge for day 8"),
        // 9 => println!("Challenge for day 9"),
        // 10 => println!("Challenge for day 10"),
        // 11 => println!("Challenge for day 11"),
        // 12 => println!("Challenge for day 12"),
        // 13 => println!("Challenge for day 13"),
        // 14 => println!("Challenge for day 14"),
        // 15 => println!("Challenge for day 15"),
        // 16 => println!("Challenge for day 16"),
        // 17 => println!("Challenge for day 17"),
        // 18 => println!("Challenge for day 18"),
        // 19 => println!("Challenge for day 19"),
        // 20 => println!("Challenge for day 20"),
        // 21 => println!("Challenge for day 21"),
        // 22 => println!("Challenge for day 22"),
        // 23 => println!("Challenge for day 23"),
        // 24 => println!("Challenge for day 24"),
        // 25 => println!("Challenge for day 25"),
        _ => DayResult{
            part_one: Err(String::from("No correct day")),
            part_two: Err(String::from("No correct day"))
        },
    };
    return result;
}


