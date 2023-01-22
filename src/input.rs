use serde::Deserialize;
use crate::Modes;

// use std::collections::HashMap;
use std::fs;


#[derive(Clone, Deserialize, Debug)]
pub struct Challenge {
    pub one: Option<String>,
    pub two: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct SingleDay {
    pub title: String,
    pub day: u32,
    pub test: Challenge,
    pub challenge: Challenge,
}

impl SingleDay {
    pub fn get_input(self, mode: Modes) -> Challenge {
        match mode {
            Modes::Test => self.test,
            Modes::Challenge => self.challenge,
        }
    }
}

pub fn parse_input(day: u8) -> SingleDay {
    // let file_path: String = String::from("./inputs/day1.toml");
    debug!("Parsing config for day {}", day);
    let file_path: String = format!("./inputs/day{}.toml", day);

    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let value: SingleDay = toml::from_str(&content).unwrap();

    return value;
}

#[cfg(test)]
mod input_test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let actual_output: SingleDay = parse_input(0);
        assert_eq!(String::from("Day Zero"), actual_output.title);
        assert_eq!(0, actual_output.day);
        assert_eq!(Some(String::from("7\n8\n9\n")), actual_output.challenge.one);
        assert_eq!(Some(String::from("1\n2\n3\n")), actual_output.challenge.two);
        assert_eq!(Some(String::from("1\n2\n3\n")), actual_output.test.one);
        assert_eq!(Some(String::from("4\n5\n6\n")), actual_output.test.two);
    }
}
