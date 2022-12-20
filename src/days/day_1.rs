use crate::days::day_result::DayResult;
use crate::input::*;
use std::collections::HashMap;

fn part_one(input: Option<String>) -> Result<String, String>{
    let plain_input: String;
    let result: Result<String, String>;

    match input {
        Some(i) => plain_input = i,
        None => {
            // result = Err("No input given".to_string());
            plain_input = "".to_string();
        },
    }

    let split = plain_input.split("\n");
    let mut elfs: HashMap<u32, u32> = HashMap::new();

    let mut current_elf: u32 = 1;
    for s in split {
        if s == "" {
            current_elf+=1;
        } else {
            let additional_value: u32 = s.parse().unwrap();

            elfs.entry(current_elf)
                .and_modify(|value| *value += additional_value)
                .or_insert(additional_value);
        }
    }
    let mut elfs_vec: Vec<(&u32, &u32)> = elfs.iter().collect();
    elfs_vec.sort_by(|a, b| a.1.cmp(&b.1));
    let highest_value = elfs_vec.last();
    match highest_value {
        Some(i) => result = Ok(i.1.to_string()),
        None => result = Err(String::from("Couldn't find a value"))
    }
    
    return result;
}

fn part_two(input: Option<String>) -> Result<String, String>{
    let plain_input: String;
    let result: Result<String, String>;

    match input {
        Some(i) => plain_input = i,
        None => {
            // result = Err("No input given".to_string());
            plain_input = "".to_string();
        },
    }

    let split = plain_input.split("\n");
    let mut elfs: HashMap<u32, u32> = HashMap::new();

    let mut current_elf: u32 = 1;
    for s in split {
        if s == "" {
            current_elf+=1;
        } else {
            let additional_value: u32 = s.parse().unwrap();

            elfs.entry(current_elf)
                .and_modify(|value| *value += additional_value)
                .or_insert(additional_value);
        }
    }
    let mut elfs_vec: Vec<(&u32, &u32)> = elfs.iter().collect();
    elfs_vec.sort_by(|a, b| a.1.cmp(&b.1));

    let best_three = elfs_vec.iter().rev().take(3);
    let mut sum: u32 = 0;
    for elem in best_three {
        sum += elem.1;
    }

    result = Ok(sum.to_string());

    return result;
}

pub fn day_one(input: Challenge) -> DayResult{
    // let result: DayResult;

    let result_one = part_one(input.one);
    let result_two = part_two(input.two);
    let result: DayResult = DayResult {
        one: result_one,
        two: result_two
    };
    return result;
}


#[cfg(test)]
mod test_day_one {
    use crate::parse_input;

    #[test]
    fn part_one() {
        let complete_input = parse_input(1);

        let input = complete_input.test;
        let day_one: Result<String, String> = super::part_one(input.one);
        assert_eq!(day_one.unwrap(), String::from("24000"));
    }

    #[test]
    fn part_two() {
        let complete_input = parse_input(1);

        let input = complete_input.test;
        let day_one: Result<String, String> = super::part_two(input.one);
        assert_eq!(day_one.unwrap(), String::from("45000"));
    }
}

