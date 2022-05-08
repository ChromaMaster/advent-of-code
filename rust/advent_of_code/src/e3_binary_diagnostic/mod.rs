use std::fs;

pub fn execute() {
    let contents = fs::read_to_string("../inputs/e3_binary_diagnostic_input.txt").expect("Cannot read the file");

    let report: Vec<String> = contents.lines().map(|s| s.to_string()).collect();

    let most_repeated_bits = get_most_repeated_bit(report);
    let less_repeated_bits = invert_binary_string(&most_repeated_bits);

    let gamma_rate = int_from_binary_string(most_repeated_bits);
    let epsilon_rate = int_from_binary_string(less_repeated_bits);

    println!("--- Part One ---");
    println!("The power consumption of the submarine is: {}", calculate_power_consumption(gamma_rate, epsilon_rate));
}

fn get_most_repeated_bit(input: Vec<String>) -> String {
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
        if x > 0 {
            String::from("1")
        } else {
            String::from("0")
        }
    }).collect();
}

fn int_from_binary_string(input: String) -> u32 {
    return u32::from_str_radix(input.as_str(), 2).expect("Not a binary number");
}

fn calculate_power_consumption(gamma_rate: u32, epsilon_rate: u32) -> u32 {
    return gamma_rate * epsilon_rate;
}

fn invert_binary_string(binary_string: &String) -> String {
    binary_string.chars().map(|c| {
        return if c == '0' {
            '1'
        } else {
            '0'
        };
    }).collect()
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

        let most_repeated_bits = get_most_repeated_bit(input);
        assert_eq!(most_repeated_bits, String::from("10110"))
    }

    #[test]
    fn a_binary_string_is_correctly_converted_to_uint() {
        assert_eq!(int_from_binary_string(String::from("01100")), 12)
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
}