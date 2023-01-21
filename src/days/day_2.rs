
use crate::days::day_result::DayResult;
use crate::input::*;

/// Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
///
/// In case the Elves get hungry and need extra snacks, they need to know which Elf to ask: they'd like to know how many Calories are being carried by the Elf carrying the most Calories. In the example above, this is 24000 (carried by the fourth Elf).
fn part_one(input: Option<String>) -> Result<String, String>{
    
    let results = ResultPoints{win: 6, loose: 0, draw: 3};

    let rock_win_loose = vec![results.draw + 1, results.loose + 1, results.win + 1];
    let paper_win_loose = vec![results.win + 2, results.draw + 2, results.loose + 2];
    let scissor_win_loose = vec![results.loose + 3, results.win + 3, results.draw + 3];
    let lose_win_table = vec![rock_win_loose, paper_win_loose, scissor_win_loose];

    // A = Rock = 1
    // B = Paper = 2
    // C = Scissors = 3
    //
    // X = Rock
    // Y = Paper
    // Z = Scissors
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
    // let plain_input: String;
    // let result: Result<String, String>;
    //
    // match input {
    //     Some(i) => plain_input = i,
    //     None => {
    //         // result = Err("No input given".to_string());
    //         plain_input = "".to_string();
    //     },
    // }
    //
    let result = Ok("test".to_string());
    return result;
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
}

