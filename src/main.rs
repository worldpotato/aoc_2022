pub mod days;
pub mod parameters;
pub mod input;

use crate::days::*;
use crate::input::*;

pub enum Modes {
    Challenge,
    Test
}

fn main() {

    let day: u8 = 2;
    let mode = Modes::Challenge;
    // let args: Vec<String> = env::args().collect();
    // let (day, mode) = parse_parameters(&args);
    let complete_input = parse_input(day);
    let input_value = complete_input.get_input(mode);
    // mode.map(|n| n.
    let day_result = run_day(day, input_value);

    match day_result.part_one {
        Ok(i) => println!("Result of part one: {}", i),
        Err(i) => println!("{}", i),
    }

    match day_result.part_two {
        Ok(i) => println!("Result of part two: {}", i),
        Err(i) => println!("{}", i),
    }
}

