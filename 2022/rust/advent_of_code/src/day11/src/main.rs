// #![deny(unused)]

use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::fs;
use regex::Regex;
use crate::Operation::Unknown;

#[derive(PartialEq, Debug)]
enum Operation {
    Mul(i64),
    Sum(i64),
    Sq,
    Unknown(String),
}

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
        Some(item / 3)
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

fn main() {
    let input = fs::read_to_string("src/day11/input/input.txt")
        .expect("Cannot open input file");

    let monkey_strings = input.trim().split("\n\n").collect::<Vec<&str>>();

    // Create the monkeys
    let mut monkeys = Vec::new();
    for m in monkey_strings {
        monkeys.push(Monkey::from(m));
    }

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while process_monkey(&mut monkeys, i).is_ok() {}
        }
    }

    let mut items_count = monkeys.iter().map(|m| (m.name.to_string(), m.items_count)).collect::<Vec<(String, u32)>>();
    items_count.sort_by(|a, b| b.1.cmp(&a.1));

    println!("Part one: The level of business after 20 rounts is {}", items_count[0].1 * items_count[1].1);
}

fn process_monkey(monkeys: &mut [Monkey], m: usize) -> Result<(), ()> {
    let monkey = monkeys.get_mut(m).unwrap();
    let item = monkey.get_item();
    if item.is_none() {
        return Err(());
    }

    let next = monkey.get_next_monkey(item.unwrap());
    monkeys.get_mut(next as usize).unwrap().add_item(item.unwrap());
    Ok(())
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
        assert_eq!(monkey.get_item().unwrap(), (79 * 19) / 3);
    }

    #[test]
    fn target_moneky_can_be_obtained() {
        let mut monkey = Monkey::from(monkey0());
        let item = monkey.get_item().unwrap();
        assert_eq!(monkey.get_next_monkey(item), 3);
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
