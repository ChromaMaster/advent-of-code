pub fn run_a(input: &Vec<String>) -> String {
    let mut result = 0;

    for line in input.iter() {
        let card_values = line.split(':').last().unwrap().trim();
        let scratchcard = Scratchcard::from(card_values);
        result += scratchcard.value();
    }

    result.to_string()
}

pub fn run_b(input: &Vec<String>) -> String {
    let result = 0;

    result.to_string()
}

struct Scratchcard {
    winning_numbers: Vec<u32>,
    scratched_numbers: Vec<u32>,
}

impl From<&str> for Scratchcard {
    fn from(input: &str) -> Self {
        let mut numbers = input.split('|');

        let winning_numbers = num_str_to_vec(numbers.next().unwrap());
        let scratched_numbers = num_str_to_vec(numbers.next().unwrap());

        Scratchcard {
            winning_numbers,
            scratched_numbers,
        }
    }
}

impl Scratchcard {
    fn value(&self) -> u32 {
        let mut value = 0;

        for number in &self.scratched_numbers {
            if self.winning_numbers.contains(number) {
                if value == 0 {
                    value = 1;
                } else {
                    value *= 2;
                }
            }
        }

        value
    }
}

fn num_str_to_vec(input: &str) -> Vec<u32> {
    input
        .split(' ')
        .filter(|n| !n.is_empty())
        .map(|n| n.parse::<u32>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::read_input_file;
    use super::*;

    #[test]
    fn given_a_string_with_a_list_of_numbers_is_possible_to_get_the_actual_numbers() {
        let input = "   1 2 3 4    5";
        assert_eq!(num_str_to_vec(input), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn given_an_input_it_gets_the_scratchcard_with_scratched_and_winning_numbers() {
        let scratchcard = Scratchcard::from("41 48 83 86 17 | 83 86  6 31 17  9 48 53");

        assert_eq!(scratchcard.winning_numbers, vec![41, 48, 83, 86, 17]);
        assert_eq!(scratchcard.scratched_numbers, vec![83, 86, 6, 31, 17, 9, 48, 53]);
    }

    #[test]
    fn given_a_scratchcard_is_possible_to_get_its_value() {
        let scratchcard = Scratchcard::from("41 48 83 86 17 | 83 86  6 31 17  9 48 53");

        assert_eq!(scratchcard.value(), 8);
    }

    #[test]
    fn given_a_pile_of_cards_is_possible_to_get_the_total_value() {
        let input = read_input_file("example04a.txt");

        assert_eq!(run_a(&input), "13");
    }
}
