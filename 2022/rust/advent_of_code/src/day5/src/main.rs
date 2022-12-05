use std::fs;
use regex::Regex;

#[derive(PartialEq, Debug)]
struct Instr {
    amount: usize,
    from: usize,
    to: usize,
}

impl From<String> for Instr {
    fn from(s: String) -> Self {
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let caps = re.captures(s.as_str()).unwrap();
        Instr {
            amount: caps[1].parse::<usize>().unwrap(),
            from: caps[2].parse::<usize>().unwrap(),
            to: caps[3].parse::<usize>().unwrap(),
        }
    }
}

fn main() {
    let input = fs::read_to_string("src/day5/input/input.txt")
        .expect("Cannot open input file");

    let lines = input
        .split("\n\n")
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    let (crates_stacks_draw, instructions) = (&lines[0], &lines[1]);

    // Part one
    let mut crate_stacks = obtain_crate_stacks(crates_stacks_draw);

    instructions.trim().split('\n').for_each(|instruction| {
        move_crates_crate_mover_9000(&mut crate_stacks, Instr::from(instruction.to_string()));
    });

    let stack_tops = get_stack_tops(&crate_stacks);

    println!("Part one: Stack tops are: {}", stack_tops.join(""));


    // Part two
    let mut crate_stacks = obtain_crate_stacks(crates_stacks_draw);

    instructions.trim().split('\n').for_each(|instruction| {
        move_crates_crate_mover_9001(&mut crate_stacks, Instr::from(instruction.to_string()));
    });

    let stack_tops = get_stack_tops(&crate_stacks);

    println!("Part two: Stack tops are: {}", stack_tops.join(""));
}

fn obtain_crate_stacks(crate_stacks_draw: &str) -> Vec<Vec<String>> {
    let mut crates = crate_stacks_draw
        .chars()
        .collect::<Vec<char>>()
        .chunks(4)
        .map(|c| c.iter().collect::<String>().replace([' ', '\n', '[', ']'], ""))
        .collect::<Vec<String>>();

    // Get the num of stacks from the last item of the last row.
    let stacks_num = crates[crates.len() - 1].parse::<usize>().unwrap();

    // Remove the last line
    crates.drain(crates.len() - stacks_num..crates.len());

    let mut stacks = vec![Vec::new(); stacks_num];

    for (i, c) in crates.into_iter().enumerate() {
        if c.is_empty() { continue; }
        stacks[i % stacks_num].push(c)
    }

    for stack in stacks.iter_mut() {
        stack.reverse()
    }

    stacks
}

fn move_crates_crate_mover_9000(stacks: &mut [Vec<String>], instructions: Instr) {
    for _ in 0..instructions.amount {
        let crate_to_move = stacks[instructions.from - 1].pop().unwrap();
        stacks[instructions.to - 1].push(crate_to_move);
    }
}

fn move_crates_crate_mover_9001(stacks: &mut [Vec<String>], instructions: Instr) {
    let from_stack = &mut stacks[instructions.from - 1];
    let crates_to_move = from_stack
        .drain(from_stack.len() - instructions.amount..from_stack.len())
        .collect::<Vec<String>>();

    let to_stack = &mut stacks[instructions.to - 1];
    to_stack.extend(crates_to_move.into_iter());
}

fn get_stack_tops(stacks: &Vec<Vec<String>>) -> Vec<String> {
    let mut stack_tops: Vec<String> = Vec::new();

    for stack in stacks {
        stack_tops.push(stack.last().unwrap().to_string());
    }

    stack_tops
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crates_stacks_are_obtained_from_the_crates_draw() {
        let input = String::from("    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 ");

        let crate_stacks: Vec<Vec<String>> = obtain_crate_stacks(&input);

        assert_eq!(crate_stacks[0], vec!["Z", "N"]);
        assert_eq!(crate_stacks[1], vec!["M", "C", "D"]);
        assert_eq!(crate_stacks[2], vec!["P"])
    }

    #[test]
    fn instruction_can_be_decoded() {
        let instr = Instr::from(String::from("move 1 from 2 to 1"));
        assert_eq!(instr, Instr { amount: 1, from: 2, to: 1 });

        let instr = Instr::from(String::from("move 3 from 1 to 3"));
        assert_eq!(instr, Instr { amount: 3, from: 1, to: 3 });

        let instr = Instr::from(String::from("move 2 from 2 to 1"));
        assert_eq!(instr, Instr { amount: 2, from: 2, to: 1 });

        let instr = Instr::from(String::from("move 1 from 1 to 2"));
        assert_eq!(instr, Instr { amount: 1, from: 1, to: 2 });
    }

    #[test]
    fn crates_are_moved_with_crate_mover_9000_based_on_instruction() {
        let mut crate_stacks = vec![
            vec![String::from("Z"), String::from("N")],
            vec![String::from("M"), String::from("C"), String::from("D")],
            vec![String::from("P")]];

        move_crates_crate_mover_9000(&mut crate_stacks, Instr { amount: 1, from: 2, to: 1 });

        assert_eq!(crate_stacks, vec![
            vec![String::from("Z"), String::from("N"), String::from("D")],
            vec![String::from("M"), String::from("C")],
            vec![String::from("P")]]);

        move_crates_crate_mover_9000(&mut crate_stacks, Instr { amount: 3, from: 1, to: 3 });

        assert_eq!(crate_stacks, vec![
            vec![],
            vec![String::from("M"), String::from("C")],
            vec![String::from("P"), String::from("D"), String::from("N"), String::from("Z")]]);

        move_crates_crate_mover_9000(&mut crate_stacks, Instr { amount: 2, from: 2, to: 1 });

        assert_eq!(crate_stacks, vec![
            vec![String::from("C"), String::from("M")],
            vec![],
            vec![String::from("P"), String::from("D"), String::from("N"), String::from("Z")]]);

        move_crates_crate_mover_9000(&mut crate_stacks, Instr { amount: 1, from: 1, to: 2 });

        assert_eq!(crate_stacks, vec![
            vec![String::from("C")],
            vec![String::from("M")],
            vec![String::from("P"), String::from("D"), String::from("N"), String::from("Z")]]);
    }

    #[test]
    fn get_all_stack_tops() {
        let stacks = vec![
            vec![String::from("C")],
            vec![String::from("M")],
            vec![String::from("P"), String::from("D"), String::from("N"), String::from("Z")]];

        let stack_tops = get_stack_tops(&stacks);

        assert_eq!(stack_tops, vec![String::from("C"), String::from("M"), String::from("Z")])
    }

    #[test]
    fn crates_are_moved_with_crate_mover_9001_based_on_instruction() {
        let mut crate_stacks = vec![
            vec![String::from("Z"), String::from("N")],
            vec![String::from("M"), String::from("C"), String::from("D")],
            vec![String::from("P")]];

        move_crates_crate_mover_9001(&mut crate_stacks, Instr { amount: 1, from: 2, to: 1 });

        assert_eq!(crate_stacks, vec![
            vec![String::from("Z"), String::from("N"), String::from("D")],
            vec![String::from("M"), String::from("C")],
            vec![String::from("P")]]);

        move_crates_crate_mover_9001(&mut crate_stacks, Instr { amount: 3, from: 1, to: 3 });

        assert_eq!(crate_stacks, vec![
            vec![],
            vec![String::from("M"), String::from("C")],
            vec![String::from("P"), String::from("Z"), String::from("N"), String::from("D")]]);

        move_crates_crate_mover_9001(&mut crate_stacks, Instr { amount: 2, from: 2, to: 1 });

        assert_eq!(crate_stacks, vec![
            vec![String::from("M"), String::from("C")],
            vec![],
            vec![String::from("P"), String::from("Z"), String::from("N"), String::from("D")]]);

        move_crates_crate_mover_9001(&mut crate_stacks, Instr { amount: 1, from: 1, to: 2 });

        assert_eq!(crate_stacks, vec![
            vec![String::from("M")],
            vec![String::from("C")],
            vec![String::from("P"), String::from("Z"), String::from("N"), String::from("D")]]);
    }
}
