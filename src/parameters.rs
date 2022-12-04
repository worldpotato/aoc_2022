use crate::Modes;
extern crate getopts;
use getopts::Options;

pub fn parse_parameters(arguments: &Vec<String>) -> (u8, Modes) {
    let mut opts = Options::new();
    let mut mode = Modes::Challenge;
    let program = arguments[0].clone();

    opts.optflag("t", "test", "Use the test input instead of the 'productive' input");
    opts.optopt("d", "day", "set the day to run", "DAY");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&arguments[1..]) {
        Ok(m) => { m }
        Err(_f) => { panic!() }
    };

    if matches.opt_present("t") {
        mode = Modes::Test;
    }
    if matches.opt_present("h") {
        print!("{}", opts.usage(&program));
        return (0, mode);
    }

    let day_match = matches.opt_str("day");
    let day: u8;
    match day_match {
        Some(x) => day = x.parse::<u8>().unwrap(),
        None => day = 0
    }

    return (day, mode);
}

