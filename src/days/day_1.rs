use crate::days::day_result::DayResult;
use crate::input::*;
use std::collections::HashMap;

/// Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
///
/// In case the Elves get hungry and need extra snacks, they need to know which Elf to ask: they'd like to know how many Calories are being carried by the Elf carrying the most Calories. In the example above, this is 24000 (carried by the fourth Elf).
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

/// Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?
///
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

/// Execute part one and two of day one
///
/// [Day one](https://adventofcode.com/2022/day/1)
///
/// The Elves take turns writing down the number of Calories contained by the various meals, snacks, rations, etc. that they've brought with them, one item per line. Each Elf separates their own inventory from the previous Elf's inventory (if any) by a blank line.
///
/// For example, suppose the Elves finish writing their items' Calories and end up with the following list:
///
/// ```
/// 1000
/// 2000
/// 3000
///
/// 4000
///
/// 5000
/// 6000
///
/// 7000
/// 8000
/// 9000
///
/// 10000
/// ```
///
/// This list represents the Calories of the food carried by five Elves:

/// * The first Elf is carrying food with 1000, 2000, and 3000 Calories, a total of 6000 Calories.
/// * The second Elf is carrying one food item with 4000 Calories.
/// * The third Elf is carrying food with 5000 and 6000 Calories, a total of 11000 Calories.
/// * The fourth Elf is carrying food with 7000, 8000, and 9000 Calories, a total of 24000 Calories.
/// * The fifth Elf is carrying one food item with 10000 Calories.
///
/// By the time you calculate the answer to the Elves' question, they've already realized that the Elf carrying the most Calories of food might eventually run out of snacks.
///
/// To avoid this unacceptable situation, the Elves would instead like to know the total Calories carried by the top three Elves carrying the most Calories. That way, even if one of those Elves runs out of snacks, they still have two backups.
///
/// In the example above, the top three Elves are the fourth Elf (with `24000` Calories), then the third Elf (with `11000` Calories), then the fifth Elf (with `10000` Calories). The sum of the Calories carried by these three elves is `45000`.


pub fn day_one(input: Challenge) -> DayResult{
    // let result: DayResult;

    let result_one = part_one(input.one);
    let result_two = part_two(input.two);
    let result: DayResult = DayResult {
        part_one: result_one,
        part_two: result_two
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

