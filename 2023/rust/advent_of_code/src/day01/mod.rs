const SPELLED_DIGITS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

pub fn run(input: &Vec<String>, check_spelled: bool) -> String {
    let mut result = 0;

    for line in input {
        let calibration_value = get_calibration_value(&line, check_spelled);
        result += calibration_value;
    }

    return result.to_string();
}

fn get_first_digit(input: &String, check_spelled: bool) -> Result<String, &str> {
    for i in 0..input.len() {
        if input.chars().nth(i).unwrap().is_digit(10) {
            return Ok(input.chars().nth(i).unwrap().to_string());
        }

        if !check_spelled {
            continue;
        }

        for j in 0..SPELLED_DIGITS.len() {
            if input[i..].starts_with(SPELLED_DIGITS[j]) {
                // Needs to add 1 to the index because the digits does not include "zero"
                return Ok((j + 1).to_string());
            }
        }
    }

    Err("No digit found")
}

fn get_last_digit(input: &String, check_spelled: bool) -> Result<String, &str> {
    for i in (0..input.len()).rev(){
        if input.chars().nth(i).unwrap().is_digit(10) {
            return Ok(input.chars().nth(i).unwrap().to_string());
        }

        if !check_spelled {
            continue;
        }

        for j in 0..SPELLED_DIGITS.len() {
            if input[..=i].ends_with(SPELLED_DIGITS[j]) {
                // Needs to add 1 to the index because the digits does not include "zero"
                return Ok((j + 1).to_string());
            }
        }
    }

    Err("No digit found")
}

fn get_calibration_value(input: &String, check_spelled: bool) -> u32 {
    let first_digit = get_first_digit(input, check_spelled).unwrap();
    let last_digit = get_last_digit(input, check_spelled).unwrap();
    let calibration_value = format!("{}{}", first_digit, last_digit);
    calibration_value.parse::<u32>().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::read_input_file;
    use super::*;

    #[test]
    fn is_possible_to_get_the_first_digit_of_a_string() {
        let input = String::from("pqr3stu8vwx");
        let expected = String::from("3");
        assert_eq!(get_first_digit(&input, false).unwrap(), expected);
    }

    #[test]
    fn is_possible_to_get_the_last_digit_of_a_string() {
        let input = String::from("pqr3stu8vwx");
        let expected = String::from("8");
        assert_eq!(get_last_digit(&input,false).unwrap(), expected);
    }

    #[test]
    fn is_possible_to_get_the_first_spelled_digit_of_a_string() {
        let input = String::from("zoneight234");
        let expected = String::from("1");
        assert_eq!(get_first_digit(&input, true).unwrap(), expected);
    }

    #[test]
    fn is_possible_to_get_the_last_spelled_digit_of_a_string() {
        let input = String::from("abcone2threexyz");
        let expected = String::from("3");
        assert_eq!(get_last_digit(&input, true).unwrap(), expected);
    }

    #[test]
    fn is_possible_to_get_the_number_formed_by_the_first_and_last_digits_of_a_string() {
        let input = String::from("pqr3stu8vwx");
        let expected = 38;
        assert_eq!(get_calibration_value(&input, true), expected);
    }

    #[test]
    fn is_possible_to_get_the_number_formed_by_the_first_and_last_spelled_digits_of_a_string(){
        let input = String::from("zoneight234");
        let expected = 14;
        assert_eq!(get_calibration_value(&input, true), expected);
    }

    #[test]
    fn is_possible_to_get_the_sum_of_all_calibration_values() {
        let input_file = read_input_file("example01a.txt");
        let expected = String::from("142");
        assert_eq!(run(&input_file, false), expected.to_string());
    }

    #[test]
    fn is_possible_to_get_the_sum_of_all_calibration_values_checking_also_spelled_digits() {
        let input_file = read_input_file("example01b.txt");
        let expected = String::from("281");
        assert_eq!(run(&input_file, true), expected.to_string());
    }
}
