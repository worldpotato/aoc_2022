pub mod days;
pub mod parameters;
pub mod input;
pub mod modes;

#[macro_use]
extern crate log;

use log::{debug, error, info};

use crate::days::*;
use crate::input::*;
use modes::Modes;
// use parameters::parse_parameters;
// use std::env;

fn main() {

    env_logger::init();
    let day: u8 = 1;
    let mode = Modes::Challenge;
    // let args: Vec<String> = env::args().collect();
    // let (day, mode) = parse_parameters(&args);
    debug!("Parse Input");
    let complete_input = parse_input(day);
    debug!("Get input");
    let input_value = complete_input.get_input(mode);
    debug!("Execute day");
    let day_result = run_day(day, input_value);

    match day_result.part_one {
        Ok(i) => info!("Result of part one: {}", i),
        Err(i) => error!("{}", i),
    }

    match day_result.part_two {
        Ok(i) => info!("Result of part two: {}", i),
        Err(i) => error!("{}", i),
    }
}

