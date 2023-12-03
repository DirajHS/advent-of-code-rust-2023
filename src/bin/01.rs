use std::collections::HashMap;
use std::usize;advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<usize> {
    let mut total_calibration: usize = 0;
    for line in input.lines() {
        let extracted_digits = line.chars().filter(|ch| ch.is_ascii_digit())
            .collect::<String>();
        let calibration = ((extracted_digits.chars().next().unwrap().to_digit(10).unwrap() * 10) +
            extracted_digits.chars().last().unwrap().to_digit(10).unwrap()) as usize;
        total_calibration += calibration;
    }
    Some(total_calibration)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut total_calibrations = 0;
    let numbers_text: Vec<&str> = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let numbers_val: HashMap<&str, usize> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),

    ]);
    for line in input.lines() {
        let mut first_digit: usize = 0;
        let mut last_digit = 0;
        'outer: for (idx, ch) in line.char_indices() {
            if ch.is_numeric() {
                first_digit = (ch.to_digit(10).unwrap_or_default() * 10) as usize;
                break;
            } else {
                for number in numbers_text.iter().enumerate() {
                    let starts_with = line[idx..line.len()].starts_with(number.1);
                    if starts_with && numbers_val.contains_key(number.1) {
                        let key_val = numbers_val.get_key_value(number.1);
                        first_digit = key_val.unwrap().1 * 10;
                        break 'outer;
                    }
                }
            }
        }
        'outer: for (idx, ch) in line.chars().rev().collect::<String>().char_indices() {
            if ch.is_numeric() {
                last_digit = (ch.to_digit(10).unwrap_or_default()) as usize;
                break;
            } else {
                for number in numbers_text.iter().enumerate() {
                    let reversed = number.1.chars().rev().collect::<String>();
                    if line.chars().rev().collect::<String>()[idx..line.len()].starts_with(&reversed) {
                        last_digit = numbers_val[number.1];
                        break 'outer;
                    }
                }
            }
        }
        total_calibrations += first_digit + last_digit;
        //println!("Calibration for {} is {}", line, (first_digit + last_digit));
    }
    Some(total_calibrations)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(479));
    }
}
