use std::fs;
use std::io::Error;

fn main() {
    let result = calibration_values("resources/input1");
    println!("day1 part 1 {:?}", result);
}

fn calibration_values(file_name: &str) -> Result<i32, Error> {
    let contents = fs::read_to_string(file_name)?;
    let result = contents
        .lines()
        .fold(0, |acc, line| acc + calibration_value(line));
    Ok(result)
}

fn calibration_value(contents: &str) -> i32 {
    let result = contents.chars().fold(Vec::new(), |mut acc, c| {
        if c.is_digit(10) {
            if acc.len() == 0 {
                acc.push(c);
                acc.push(c);
            } else {
                acc[1] = c;
            }
        }
        acc
    });
    if result.len() != 2 {
        return 0;
    }
    let first_two: String = result[0..2].iter().collect();
    first_two.parse::<i32>().unwrap_or(0)
}

#[cfg(test)]
mod tests_day1_part1 {

    use crate::calibration_value;
    use crate::calibration_values;

    #[test]
    fn test_calibration_value() {
        assert_eq!(calibration_value("1abc2"), 12);
        assert_eq!(calibration_value("pqr3stu8vwx"), 38);
        assert_eq!(calibration_value("a1b2c3d4e5f"), 15);
        assert_eq!(calibration_value("treb7uchet"), 77);
    }

    #[test]
    fn test_calibration_values_sample_sum() {
        let result = calibration_values("resources/sample.txt");
        assert!(result.is_ok(), "Expected OK, got {:?}", result);
        let result = result.unwrap();
        assert!(result == 142, "Expected 142, got {}", result);
    }

    #[test]
    fn test_input1() {
        let result = calibration_values("resources/input1");
        assert!(result.is_ok(), "Expected OK, got {:?}", result);
        let expected = 55029;
        let result = result.unwrap();
        assert!(result == 55029, "Expected {}, got {}", expected, result);
    }
}
