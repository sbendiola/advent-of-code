use std::fs;
use std::io::Error;

const DEBUG: bool = false;

const WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let result = calibration_values("resources/input1");
    println!("day 1 part2 {:?}", result);
}

fn calibration_values(file_name: &str) -> Result<i32, Error> {
    let contents = fs::read_to_string(file_name)?;
    let result = contents.lines().fold(0, |acc, line| {
        let result = calibration_value_for_line(line);
        if DEBUG {
            println!("line: {} result:{}", line, result);
        }
        return acc + result;
    });
    Ok(result)
}

fn calibration_value_for_line(text: &str) -> i32 {
    struct Result {
        first: Option<(i32, usize)>,
        last: Option<(i32, usize)>,
    }
    let mut result: Result = Result {
        first: None,
        last: None,
    };

    for (index, c) in text.chars().enumerate() {
        if c.is_digit(10) {
            let value = c.to_digit(10).unwrap() as i32;
            if value > 0 {
                if result.first.is_none() {
                    result.first = Some((value, index));
                }
                result.last = Some((value, index));
            }
        }
    }

    for (index, word) in WORDS.iter().enumerate() {
        let mut start = 0;
        while let Some(word_index) = text[start..].find(word) {
            let real_index = start + word_index;
            if result.first.is_none() || real_index < result.first.unwrap().1 {
                result.first = Some((index as i32 + 1, real_index));
            }
            if result.last.is_none() || real_index > result.last.unwrap().1 {
                result.last = Some((index as i32 + 1, real_index));
            }
            start = real_index + word.len();
        }
    }

    assert!(result.first.unwrap().1 <= result.last.unwrap().1);
    result.first.unwrap().0 * 10 + result.last.unwrap().0
}

#[cfg(test)]
mod tests_day1_part2 {
    use crate::calibration_value_for_line;
    use crate::calibration_values;

    #[test]
    fn test_expected_value() {
        let result = calibration_values("resources/input1");
        assert!(result.is_ok(), "Expected OK, got {:?}", result);
        let expected = 55686;
        let result = result.unwrap();
        assert!(result == expected, "Expected {}, got {}", expected, result);
    }
    #[test]
    fn test_calibration_value_for_line() {
        assert_eq!(calibration_value_for_line("two1nine"), 29);
        assert_eq!(calibration_value_for_line("eightwothree"), 83);
        assert_eq!(calibration_value_for_line("abcone2threexyz"), 13);
        assert_eq!(calibration_value_for_line("xtwone3four"), 24);
        assert_eq!(calibration_value_for_line("4nineeightseven2"), 42);
        assert_eq!(calibration_value_for_line("zoneight234"), 14);
        assert_eq!(calibration_value_for_line("7pqrstsixteen"), 76);
        assert_eq!(calibration_value_for_line("jbmmvjgkt288"), 28);
        assert_eq!(calibration_value_for_line("oneight"), 18);
        assert_eq!(calibration_value_for_line("29oneightt"), 28);
        assert_eq!(calibration_value_for_line("eighthree"), 83);
        assert_eq!(calibration_value_for_line("sevenine"), 79);
        assert_eq!(calibration_value_for_line("xtwone3four"), 24);
        assert_eq!(calibration_value_for_line("2twonemg "), 21);
        assert_eq!(calibration_value_for_line("2twonemg2"), 22);
        assert_eq!(calibration_value_for_line("one9"), 19);
        assert_eq!(calibration_value_for_line("onenine"), 19);
        assert_eq!(calibration_value_for_line("19"), 19);
        assert_eq!(calibration_value_for_line("ninenine"), 99);
        assert_eq!(calibration_value_for_line("nine0"), 99);
    }
}
