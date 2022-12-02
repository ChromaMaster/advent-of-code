pub mod game;

use std::fs;
use game::*;

const SCORE_IF_DRAW: u32 = 3;
const SCORE_IF_WIN: u32 = 6;
const SCORE_IF_LOSE: u32 = 0;

fn main() {
    let input = fs::read_to_string("src/day2/input/input.txt")
        .expect("Cannot open input file");

    let games = input.trim().split('\n').map(|game| (game, evaluate_game(game.to_owned())))
        .collect::<Vec<(&str, u32)>>();

    let total_score = games.iter().fold(0, |acc, game| acc + game.1);
    println!("Part one: Total score is: {}", total_score);

    let games = input.trim().split('\n').map(|game| (apply_elf_strategy(game.to_owned()), evaluate_game(apply_elf_strategy(game.to_owned()))))
        .collect::<Vec<(String, u32)>>();
    let total_score = games.iter().fold(0, |acc, game| acc + game.1);
    println!("Part Two: Total score when applying elf's strategy is: {}", total_score);
}

fn evaluate_game(game: String) -> u32 {
    let g = game.split(' ')
        .map(|hand| Hand::from(hand.chars().collect::<Vec<char>>()[0]))
        .collect::<Vec<Hand>>();

    let (hand1, hand2) = (&g[0], &g[1]);

    let mut score = match hand2 {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
        _ => 0
    };

    score += if hand1 == hand2 { SCORE_IF_DRAW } else if hand2 > hand1 { SCORE_IF_WIN } else { SCORE_IF_LOSE };

    score
}

fn apply_elf_strategy(game: String) -> String {
    let g = game.split(' ').map(|hand| hand.chars().collect::<Vec<char>>()[0])
        .collect::<Vec<char>>();

    let hand1 = Hand::from(g[0]);
    let hand2 = match g[1] {
        'X' => {
            match hand1 {
                Hand::Rock => 'C',
                Hand::Paper => 'A',
                Hand::Scissors => 'B',
                _ => '?'
            }
        }
        'Y' => {
            g[0]
        }
        'Z' => {
            match hand1 {
                Hand::Rock => 'B',
                Hand::Paper => 'C',
                Hand::Scissors => 'A',
                _ => '?'
            }
        }
        _ => '?'
    };

    format!("{} {}", g[0], hand2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_score_is_calculated() {
        let score = evaluate_game(String::from("A Y"));
        assert_eq!(score, 8);

        let score = evaluate_game(String::from("B X"));
        assert_eq!(score, 1);

        let score = evaluate_game(String::from("C Z"));
        assert_eq!(score, 6);
    }

    #[test]
    fn elf_strategy_is_applied() {
        let updated_game = apply_elf_strategy(String::from("A X"));
        assert_eq!(updated_game, "A C");

        let updated_game = apply_elf_strategy(String::from("A Y"));
        assert_eq!(updated_game, "A A");

        let updated_game = apply_elf_strategy(String::from("A Z"));
        assert_eq!(updated_game, "A B");

        let updated_game = apply_elf_strategy(String::from("B X"));
        assert_eq!(updated_game, "B A");

        let updated_game = apply_elf_strategy(String::from("B Y"));
        assert_eq!(updated_game, "B B");

        let updated_game = apply_elf_strategy(String::from("B Z"));
        assert_eq!(updated_game, "B C");

        let updated_game = apply_elf_strategy(String::from("C X"));
        assert_eq!(updated_game, "C B");

        let updated_game = apply_elf_strategy(String::from("C Y"));
        assert_eq!(updated_game, "C C");

        let updated_game = apply_elf_strategy(String::from("C Z"));
        assert_eq!(updated_game, "C A");
    }
}
