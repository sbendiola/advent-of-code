use std::fs;
use std::io::Error;
use std::collections::HashMap;

const debug: bool = true;

fn main() {
    println!("Hello, world!");
}

fn calibration_values(file_name: &str) -> Result<i32, Error> {
    let contents = fs::read_to_string(file_name)?;
    let result = contents
        .lines()
        .fold(0, |acc, line| {
            let result = calibration_value(line);
            println!("line: {} result:{}", line, result);
            return acc + result
        });
    Ok(result)
}

fn indexes(text: &str) -> Vec<(i32, usize)> {
    let mut result = Vec::new();
    for (index, c) in text.chars().enumerate() {
        if c.is_digit(10) {
            let value = c.to_digit(10).unwrap() as i32;
            if value > 0 {
                result.push((value, index));
            }
        } 
    }

    let words = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    for (index, word) in words.iter().enumerate() {
        let mut start = 0;
        while let Some(word_index) = text[start..].find(word) {
            let real_index = start + word_index;
            result.push((index as i32 + 1, real_index));
            start = real_index + word.len();
        }
    }

    result.sort_by(|a, b| a.1.cmp(&b.1));
    result
}

fn calibration_value_from_vec(contents: Vec<(i32, usize)>) -> i32 {
    assert!(contents.len() > 0, "Expected contents to have at least one element, got {:?}", contents);
    contents[0].0 * 10 + contents[contents.len() - 1].0
}

fn calibration_value(contents: &str) -> i32 {
    let indices = indexes(contents);
    if debug {
        println!("indices: {:?}", indices);
    }
    return calibration_value_from_vec(indices);
}

#[cfg(test)]
mod tests {
    use crate::calibration_value;
    use crate::calibration_values;
    use crate::indexes;
    use crate::calibration_value_from_vec;

    #[test]
    fn test_calibration_value() {
        assert_eq!(calibration_value("two1nine"), 29);
        assert_eq!(calibration_value("eightwothree"), 83);
        assert_eq!(calibration_value("abcone2threexyz"), 13);
        assert_eq!(calibration_value("xtwone3four"), 24);
        assert_eq!(calibration_value("4nineeightseven2"), 42);
        assert_eq!(calibration_value("zoneight234"), 14);
        assert_eq!(calibration_value("7pqrstsixteen"), 76);
        assert_eq!(calibration_value("jbmmvjgkt288"), 28);
        assert_eq!(calibration_value("hvmbmqnxk4onesix29kdhrdqtcfx1znmjhfjx"), 41);
        assert_eq!(calibration_value("oneight"), 18);
        assert_eq!(calibration_value("three2fiveonexrllxsvfive"), 35);
    }
    

    #[test]
    fn test_indexes() {
        assert_eq!(indexes("two1nine"), vec![(2, 0), (1, 3), (9, 4)]);
        assert_eq!(calibration_value_from_vec(indexes("two1nine")), 29);
        assert_eq!(calibration_value_from_vec(indexes("eightwothree")), 83);
        assert_eq!(calibration_value_from_vec(indexes("abcone2threexyz")), 13);
        assert_eq!(calibration_value_from_vec(indexes("xtwone3four")), 24);
        assert_eq!(calibration_value_from_vec(indexes("4nineeightseven2")), 42);
        assert_eq!(calibration_value_from_vec(indexes("zoneight234")), 14);
        assert_eq!(calibration_value_from_vec(indexes("7pqrstsixteen")), 76);
        assert_eq!(calibration_value_from_vec(indexes("jbmmvjgkt288")), 28);
        assert_eq!(calibration_value_from_vec(indexes("hvmbmqnxk4onesix29kdhrdqtcfx1znmjhfjx")), 41);
        assert_eq!(calibration_value_from_vec(indexes("oneight")), 18);
        assert_eq!(calibration_value_from_vec(indexes("29oneightt")), 28);
        assert_eq!(calibration_value_from_vec(indexes("eighthree")), 83);
        assert_eq!(calibration_value_from_vec(indexes("sevenine")), 79);
        assert_eq!(calibration_value_from_vec(indexes("xtwone3four")), 24);
        assert_eq!(calibration_value_from_vec(indexes("8threeesevennfourrgbgteightt5twooneenjr")), 81);
        assert_eq!(calibration_value_from_vec(indexes("2twonemg ")), 21);
        assert_eq!(calibration_value_from_vec(indexes("2twonemg2")), 22);
        assert_eq!(calibration_value_from_vec(indexes("three2twonemgthree")), 33);
        assert_eq!(calibration_value_from_vec(indexes("one9")), 19);
        assert_eq!(calibration_value_from_vec(indexes("onenine")), 19);
        assert_eq!(calibration_value_from_vec(indexes("19")), 19);
        assert_eq!(calibration_value_from_vec(indexes("ninenine")), 99);
        assert_eq!(calibration_value_from_vec(indexes("nine0")), 99);
        
        let result = calibration_values("resources/input1");
        assert!(result.is_ok(), "Expected OK, got {:?}", result);
        let expected = 55686;
        let result = result.unwrap();
        assert!(result == expected, "Expected {}, got {}", expected, result);
    }


}
