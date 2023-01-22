
use crate::days::day_result::DayResult;
use crate::input::*;

use log::{debug, error, log_enabled, info, Level};

/// Calculate the value from the given hand.
///
/// Expect either a-c or x - z
fn calculate_value_from_input(input: u8) -> u8 {

    
    info!("input: {}", input);
    // the offset for the chars a, b, c
    let beginning_offset = 64;
    // the offset for the chars x, y , z
    let ending_offset = 87;

    let mut hand_value = input - beginning_offset;


    println!("hand: {}", hand_value);
    // here we distingues between the begin and ending
    if hand_value > 3 {
        let diff = ending_offset - beginning_offset;
        println!("{}", diff);
        hand_value -= diff;
    }

    return hand_value;
}

/// Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
///
/// In case the Elves get hungry and need extra snacks, they need to know which Elf to ask: they'd like to know how many Calories are being carried by the Elf carrying the most Calories. In the example above, this is 24000 (carried by the fourth Elf).
fn part_one(input: Option<String>) -> Result<String, String>{
    
    let results = ResultPoints{win: 6, loose: 0, draw: 3};

    let rock_win_loose = vec![results.draw + 1, results.loose + 1, results.win + 1];
    let paper_win_loose = vec![results.win + 2, results.draw + 2, results.loose + 2];
    let scissor_win_loose = vec![results.loose + 3, results.win + 3, results.draw + 3];
    let lose_win_table = vec![rock_win_loose, paper_win_loose, scissor_win_loose];

    // a = rock = 1
    // b = paper = 2
    // c = scissors = 3
    //
    // x = rock
    // y = paper
    // z = scissors
    //
    // lost = 0
    // draw = 3
    // won = 6

    let elf_offset = 64;
    let my_offset = 87;

    let plain_input = match input {
        Some(i) => i,
        None => {
            // result = Err("No input given".to_string());
            "No input".to_string()
        },
    };
    let lines = plain_input.split('\n');

    let mut result_sum: u32 = 0;

    for line in lines {

        let is_ascii = line.is_ascii();
        if is_ascii{
            let bytes = line.as_bytes();

            let elfs_hand = bytes[0];
            let my_hand = bytes[2];

            let elfs_hand_value = elfs_hand - elf_offset;
            let my_hand_value = my_hand - my_offset;

            let elf_index = usize::from(elfs_hand_value - 1);
            let my_index = usize::from(my_hand_value - 1);

            let fight_result = &lose_win_table[my_index][elf_index];

            // result_sum += 
            result_sum += u32::from(*fight_result);
        }
    }

    let my_result: Result<String, String> = Ok(result_sum.to_string());
    return my_result;
}

/// Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?
///
fn part_two(input: Option<String>) -> Result<String, String>{

        
    // let results = ResultPoints {win: 6, loose: 0, draw: 3};
    //
    // let rock_win_loose = vec![results.draw + 1, results.loose + 1, results.win + 1];
    // let paper_win_loose = vec![results.win + 2, results.draw + 2, results.loose + 2];
    // let scissor_win_loose = vec![results.loose + 3, results.win + 3, results.draw + 3];
    // let lose_win_table = vec![rock_win_loose, paper_win_loose, scissor_win_loose];
    //
    // // a = rock = 1
    // // b = paper = 2
    // // c = scissors = 3
    // //
    // // x = loose
    // // y = draw
    // // z = win
    // //
    // // lost = 0
    // // draw = 3
    // // won = 6
    //
    // let elf_offset = 64;
    // let my_offset = 87;
    //
    // let plain_input = match input {
    //     some(i) => i,
    //     none => {
    //         // result = err("no input given".to_string());
    //         "no input".to_string()
    //     },
    // };
    // let lines = plain_input.split('\n');
    //
    let my_result: Result<String, String> = Ok(String::from("Test"));
    return my_result;
}

pub fn day_two(input: Challenge) -> DayResult{
    // let result: DayResult;

    let result_one = part_one(input.one);
    let result_two = part_two(input.two);
    let result: DayResult = DayResult {
        part_one: result_one,
        part_two: result_two
    };

    // let result: DayResult = DayResult(result_one, result_two);
    return result;
}

 #[derive(Debug)]
struct ResultPoints {
    win: u8,
    loose: u8,
    draw: u8,
}


#[cfg(test)]
mod test_day_two {
    use crate::parse_input;

    #[test]
    fn part_one() {
        let complete_input = parse_input(2);

        let input = complete_input.test;
        let day_one: Result<String, String> = super::part_one(input.one);
        assert_eq!(day_one.unwrap(), String::from("15"));
    }

    #[test]
    fn part_two() {
        let complete_input = parse_input(2);

        let input = complete_input.test;
        let day_one: Result<String, String> = super::part_two(input.one);
        assert_eq!(day_one.unwrap(), String::from("45000"));
    }

    #[test]
    fn test_calculate_value_from_input(){
        let input_a = "A";
        let input_b = "B";
        let input_c = "C";

        let input_x = "X";
        let input_y = "Y";
        let input_z = "Z";
        
        let first_result = super::calculate_value_from_input(input_a.as_bytes()[0]);
        assert_eq!(first_result, 1);
        let second_result = super::calculate_value_from_input(input_b.as_bytes()[0]);
        assert_eq!(second_result, 2);
        let third_result = super::calculate_value_from_input(input_c.as_bytes()[0]);
        assert_eq!(third_result, 3);
        let fourth_result = super::calculate_value_from_input(input_x.as_bytes()[0]);
        assert_eq!(fourth_result, 1);
        let fifth_result = super::calculate_value_from_input(input_y.as_bytes()[0]);
        assert_eq!(fifth_result, 2);
        let sixed_result = super::calculate_value_from_input(input_z.as_bytes()[0]);
        assert_eq!(sixed_result, 3);
    }
}

