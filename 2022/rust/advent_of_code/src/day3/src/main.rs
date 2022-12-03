use std::collections::HashSet;
use std::fs;

struct Rucksack {
    pub compartments: Vec<String>,
}

impl Rucksack {
    pub fn new(items: String) -> Self {
        let (c0_items, c1_items) = items.split_at(items.len() / 2);

        Self {
            compartments: vec![c0_items.to_owned(), c1_items.to_owned()]
        }
    }

    pub fn get_shared_items(&self) -> Vec<char> {
        self.compartments[0]
            .chars()
            .filter(|&chr| self.compartments[1].contains(chr))
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()
    }
}

fn main() {
    let input = fs::read_to_string("src/day3/input/input.txt")
        .expect("Cannot open input file");

    let rucksacks = input.trim().split('\n').map(|line| Rucksack::new(line.to_owned())).collect::<Vec<Rucksack>>();

    let mut rucksacks_priorities: Vec<u32> = Vec::new();
    for rucksack in rucksacks {
        let shared_items = rucksack.get_shared_items();
        rucksacks_priorities.push(shared_items.iter().fold(0, |acc, &item| acc + get_item_priority(item)));
    }

    println!("Part one: Total priority sum is {}", rucksacks_priorities.iter().sum::<u32>())
}

pub fn get_item_priority(item: char) -> u32 {
    if item.is_lowercase() {
        (item as u32) - ('a' as u32) + 1
    } else {
        (item as u32) - ('A' as u32) + 27
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rucksack_is_created_correctly() {
        let rucksak = Rucksack::new(String::from("vJrwpWtwJgWrhcsFMMfFFhFp"));

        assert_eq!(rucksak.compartments[0], "vJrwpWtwJgWr");
        assert_eq!(rucksak.compartments[1], "hcsFMMfFFhFp");

        let rucksak = Rucksack::new(String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"));

        assert_eq!(rucksak.compartments[0], "jqHRNqRjqzjGDLGL");
        assert_eq!(rucksak.compartments[1], "rsFMfFZSrLrFZsSL");
    }

    #[test]
    fn is_possible_to_get_shared_items() {
        let rucksak = Rucksack::new(String::from("vJrwpWtwJgWrhcsFMMfFFhFp"));

        let shared_items = rucksak.get_shared_items();
        let expected_shared_items = vec!['p'];

        assert_eq!(shared_items, expected_shared_items);

        let rucksak = Rucksack::new(String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"));

        let shared_items = rucksak.get_shared_items();
        let expected_shared_items = vec!['L'];

        assert_eq!(shared_items, expected_shared_items);
    }

    #[test]
    fn item_priority_can_be_retrieved() {
        assert_eq!(get_item_priority('a'), 1);
        assert_eq!(get_item_priority('g'), 7);
        assert_eq!(get_item_priority('n'), 14);
        assert_eq!(get_item_priority('z'), 26);
        assert_eq!(get_item_priority('A'), 27);
        assert_eq!(get_item_priority('L'), 38);
        assert_eq!(get_item_priority('O'), 41);
        assert_eq!(get_item_priority('Z'), 52);
    }
}
