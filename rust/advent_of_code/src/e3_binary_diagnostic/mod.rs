use std::fs;

pub fn execute() {
    let contents = fs::read_to_string("../inputs/e3_binary_diagnostic_input.txt").expect("Cannot read the file");

    let report: Vec<String> = contents.lines().map(|s| s.to_string()).collect();


    println!("--- Part One ---");
    let gamma_rate = get_gamma_rate(&report);
    let epsilon_rate = get_epsilon_rate(&report);
    println!("The power consumption of the submarine is: {}", calculate_power_consumption(gamma_rate, epsilon_rate));

    println!("--- Part Two ---");
    let oxygen_generator_rating = get_oxygen_generator_rating(&report);
    let get_co2_scrubber_rating = get_co2_scrubber_rating(&report);
    println!("The life support of the submarine is: {}", calculate_life_support(oxygen_generator_rating, get_co2_scrubber_rating));

}

fn calculate_power_consumption(gamma_rate: u32, epsilon_rate: u32) -> u32 {
    gamma_rate * epsilon_rate
}

fn calculate_life_support(oxygen_generator_rating: u32, co2_scrubber_rating: u32) -> u32{
    oxygen_generator_rating * co2_scrubber_rating
}

fn get_gamma_rate(input: &Vec<String>) -> u32 {
    int_from_binary_string(&get_most_repeated_bit(input))
}

fn get_epsilon_rate(input: &Vec<String>) -> u32 {
    int_from_binary_string(&invert_binary_string(&get_most_repeated_bit(input)))
}

fn get_oxygen_generator_rating(input: &Vec<String>) -> u32 {
    let mut tmp = input.to_vec();
    for (i, _) in input.iter().enumerate() {
        tmp = filter_by_most_common_bit(&tmp, i);
        if tmp.len() == 1{
            break
        }
    }
    int_from_binary_string(&tmp[0])
}

fn get_co2_scrubber_rating(input: &Vec<String>) -> u32 {
    let mut tmp = input.to_vec();
    for (i, _) in input.iter().enumerate() {
        tmp = filter_by_less_common_bit(&tmp, i);
        if tmp.len() == 1{
            break
        }
    }
    int_from_binary_string(&tmp[0])
}

fn get_most_repeated_bit(input: &Vec<String>) -> String {
    let mut tmp = vec![0; input[0].len()];

    input.iter().for_each(|value| {
        value.chars().enumerate().for_each(|(i, c)| {
            if c == '0' {
                tmp[i] -= 1
            } else {
                tmp[i] += 1
            }
        })
    });

    return tmp.iter().map(|&x| {
        if x >= 0 {
            String::from("1")
        } else {
            String::from("0")
        }
    }).collect();
}

fn int_from_binary_string(input: &String) -> u32 {
    return u32::from_str_radix(input.as_str(), 2).expect("Not a binary number");
}

fn invert_binary_string(binary_string: &String) -> String {
    binary_string.chars().map(|c| {
        if c == '0' {
            '1'
        } else {
            '0'
        }
    }).collect()
}

fn filter_by_same_bit_in_a_position(input: &Vec<String>, mask: String, pos: usize) -> Vec<String> {
    let mut tmp = input.to_vec();
    // tmp = tmp.iter().filter(|&s| s.chars().nth(pos).unwrap() == mask.chars().nth(pos).unwrap()).collect();
    tmp.retain(|s| s.chars().nth(pos).unwrap() == mask.chars().nth(pos).unwrap());
    return tmp.to_vec();
}

fn filter_by_most_common_bit(input: &Vec<String>, pos: usize) -> Vec<String> {
    let mcb = get_most_repeated_bit(&input);
    filter_by_same_bit_in_a_position(input, mcb, pos)
}

fn filter_by_less_common_bit(input: &Vec<String>, pos: usize) -> Vec<String> {
    let lcb = invert_binary_string(&get_most_repeated_bit(&input));
    filter_by_same_bit_in_a_position(input, lcb, pos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn most_repeated_bit_is_correctly_found() {
        let input = vec!["00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let most_repeated_bits = get_most_repeated_bit(&input);
        assert_eq!(most_repeated_bits, String::from("10110"))
    }

    #[test]
    fn a_binary_string_is_correctly_converted_to_uint() {
        assert_eq!(int_from_binary_string(&String::from("01100")), 12)
    }

    #[test]
    fn power_consumption_is_calculated_correctly() {
        let gamma_rate = 22;
        let epsilon_rate = 9;
        assert_eq!(calculate_power_consumption(gamma_rate, epsilon_rate), 198)
    }

    #[test]
    fn binary_string_can_be_inverted() {
        let binary_string = String::from("01010");
        assert_eq!(invert_binary_string(&binary_string), String::from("10101"))
    }

    #[test]
    fn filter_binary_numbers_by_most_common_bit_in_a_certain_position() {
        let input = vec!["00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let filtered = filter_by_most_common_bit(&input, 0);
        assert_eq!(filtered, vec!["11110", "10110", "10111", "10101", "11100", "10000", "11001"])
    }

    #[test]
    fn filter_binary_numbers_by_less_common_bit_in_a_certain_position() {
        let input = vec!["00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let filtered = filter_by_less_common_bit(&input, 0);
        assert_eq!(filtered, vec!["00100", "01111", "00111", "00010", "01010"])
    }

    #[test]
    fn oxygen_generator_rating_is_correctly_calculated() {
        let input = vec!["00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let oxygen_generator_rating = get_oxygen_generator_rating(&input);
        assert_eq!(oxygen_generator_rating, 23);
    }

    #[test]
    fn co2_scrubber_rating_is_correctly_calculated() {
        let input = vec!["00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let oxygen_generator_rating = get_co2_scrubber_rating(&input);
        assert_eq!(oxygen_generator_rating, 10);
    }
}