// #![deny(unused)]

use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::fs;
use regex::Regex;
use crate::Operation::Unknown;

#[derive(PartialEq, Debug, Clone)]
enum Operation {
    Mul(i64),
    Sum(i64),
    Sq,
    Unknown(String),
}

#[derive(Clone)]
struct Monkey {
    name: String,
    items: VecDeque<i64>,
    op: Operation,
    test: i64,
    next_monkey: Vec<u32>,

    items_count: u32,
}

impl From<&str> for Monkey {
    fn from(input: &str) -> Self {
        let re = Regex::new(r"Monkey (\d+):
  Starting items: ((?:\d+,? ?)+)
  Operation: new = (\w+) ([*+]) (\w+)
  Test: divisible by (\d+)
    If true: throw to monkey (\d+)
    If false: throw to monkey (\d+)").unwrap();

        let caps = re.captures(input).unwrap();

        let mut m = Monkey {
            name: format!("Monkey {}", &caps[1]),
            items: VecDeque::new(),
            op: Operation::Sq,
            test: caps[6].parse::<i64>().unwrap(),
            next_monkey: vec![caps[8].parse::<u32>().unwrap(), caps[7].parse::<u32>().unwrap()],
            items_count: 0,
        };


        // Get starting items
        for item in caps[2].replace(',', "").split(' ') {
            m.items.push_back(item.parse::<i64>().unwrap());
        }

        m.op = match &caps[4] {
            "*" => {
                if &caps[5] == "old" {
                    Operation::Sq
                } else {
                    Operation::Mul(caps[5].parse::<i64>().unwrap())
                }
            }
            "+" => {
                Operation::Sum(caps[5].parse::<i64>().unwrap())
            }
            _ => { Unknown(String::from(&caps[4])) }
        };

        m
    }
}

impl Monkey {
    pub fn add_item(&mut self, item: i64) {
        self.items.push_back(item);
    }

    pub fn get_item(&mut self) -> Option<i64> {
        let mut item = self.items.pop_front()?;

        item = match self.op {
            Operation::Sum(v) => item + v,
            Operation::Mul(v) => item * v,
            Operation::Sq => item.pow(2),
            _ => 1
        };

        self.items_count += 1;
        Some(item)
    }

    pub fn get_next_monkey(&self, item: i64) -> u32 {
        self.next_monkey[(item % self.test == 0) as usize]
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Monkey {}: {:?}", self.name, self.items)
    }
}

struct KeepAwayGame {
    monkeys: Vec<Monkey>,

    total_rounds: u32,
    current_round: u32,

    stress_divider: i64,
}

impl KeepAwayGame {
    pub fn new(monkeys: Vec<Monkey>, total_rounds: u32, stress_divider: i64) -> Self {
        Self { monkeys, total_rounds, stress_divider, current_round: 0 }
    }

    pub fn process_round(&mut self) -> Result<(), ()> {
        if self.current_round >= self.total_rounds {
            return Err(());
        }

        for m in 0..self.monkeys.len() {
            while self.process_monkey(m).is_ok() {};
        }

        self.current_round += 1;
        Ok(())
    }

    pub fn process_monkey(&mut self, m: usize) -> Result<(), ()> {
        let current_monkey = self.monkeys.get_mut(m).unwrap();
        let item = current_monkey.get_item();
        if item.is_none() {
            return Err(());
        }

        let mut item_value = item.unwrap() / self.stress_divider;
        let next = current_monkey.get_next_monkey(item_value);
        self.monkeys.get_mut(next as usize).unwrap().add_item(item_value);

        Ok(())
    }

    pub fn get_monkeys_total_business(&self) -> u32 {
        let mut items_count = self.monkeys
            .iter()
            .map(|m| (m.name.to_string(), m.items_count))
            .collect::<Vec<(String, u32)>>();
        items_count.sort_by(|a, b| b.1.cmp(&a.1));

        items_count[0].1 * items_count[1].1
    }
}

fn main() {
    let input = fs::read_to_string("src/day11/input/input.txt")
        .expect("Cannot open input file");

    let monkey_strings = input.trim().split("\n\n").collect::<Vec<&str>>();

    // Create the monkeys
    let mut monkeys = Vec::new();
    for m in monkey_strings {
        monkeys.push(Monkey::from(m));
    }

    // Part one
    let mut game = KeepAwayGame::new(monkeys.clone(), 20, 3);
    while game.process_round().is_ok() {};
    println!("Part one: The level of business after 20 rounds is {}", game.get_monkeys_total_business());

    // Part two
    // FIXME: This overflows as the worry level is not taken into account. There are some solutions out there using
    //  the modulo to reduce the item worry level, but I'm not sure how to implement it.
    let mut game = KeepAwayGame::new(monkeys.clone(), 1000, 1);
    while game.process_round().is_ok() {};
    println!("Part two: The level of business after 1000 rounds is {}", game.get_monkeys_total_business());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monkey_is_created_from_input() {
        let monkey = Monkey::from(monkey0());

        assert_eq!(monkey.items, vec![79, 98]);
        assert_eq!(monkey.op, Operation::Mul(19));
        assert_eq!(monkey.test, 23);
        assert_eq!(monkey.next_monkey, vec![3, 2]);
    }

    #[test]
    fn an_item_can_be_added_to_a_monkey() {
        let mut monkey = Monkey::from(monkey0());
        monkey.add_item(1);
        assert_eq!(monkey.items, vec![79, 98, 1]);
    }

    #[test]
    fn an_item_can_be_obtained_from_a_monkey() {
        let mut monkey = Monkey::from(monkey0());
        assert_eq!(monkey.get_item().unwrap(), (79 * 19));
    }

    #[test]
    fn target_moneky_can_be_obtained_after_relieving_stress() {
        let mut monkey = Monkey::from(monkey0());
        let item = monkey.get_item().unwrap();
        assert_eq!(monkey.get_next_monkey(item / 3), 3);
    }

    fn monkey0() -> &'static str {
        r"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3"
    }
}
