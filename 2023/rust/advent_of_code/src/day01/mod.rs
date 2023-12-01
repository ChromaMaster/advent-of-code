pub fn run(input: Vec<String>) -> String {
    let mut result = 0;

    for line in input {
        let calibration_value = get_calibration_value(&line);
        result += calibration_value;
    }

    return result.to_string();
}

fn get_first_digit(input: &String) -> String {
    input.chars().filter(|c| c.is_digit(10)).take(1).collect()
}

fn get_last_digit(input: &String) -> String {
    input.chars().filter(|c| c.is_digit(10)).last().unwrap().to_string()
}

fn get_calibration_value(input: &String) -> u32 {
    let first_digit = get_first_digit(input);
    let last_digit = get_last_digit(input);
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
        assert_eq!(get_first_digit(&input), expected);
    }

    #[test]
    fn is_possible_to_get_the_last_digit_of_a_string(){
        let input = String::from("pqr3stu8vwx");
        let expected = String::from("8");
        assert_eq!(get_last_digit(&input), expected);
    }

    #[test]
    fn is_possible_to_get_the_number_formed_by_the_first_and_last_digits_of_a_string(){
        let input = String::from("pqr3stu8vwx");
        let expected = 38;
        assert_eq!(get_calibration_value(&input), expected);
    }

    #[test]
    fn is_possible_to_get_the_sum_of_all_calibration_values() {
        let input_file = read_input_file("example01a.txt");
        let expected = String::from("142");
        assert_eq!(run(input_file), expected.to_string());
    }
}
