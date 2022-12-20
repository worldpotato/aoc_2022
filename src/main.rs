
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
    let day = 1;
    let mode = Modes::Challenge;
    // let args: Vec<String> = env::args().collect();
    // let (day, mode) = parse_parameters(&args);
    let complete_input = parse_input(day);
    let input;
    match mode {
        Modes::Test => input = complete_input.test,
        Modes::Challenge => input = complete_input.challenge,
    }
    let day_result = run_day(day, input);
    let day_one_result = day_result.one;
    let day_two_result = day_result.two;

    match day_one_result {
        Ok(i) => println!("Result of part one: {}", i),
        Err(i) => println!("{}", i),
    }
    match day_two_result {
        Ok(i) => println!("Result of part two: {}", i),
        Err(i) => println!("{}", i),
    }
}


