pub mod game;

use std::fs;
use game::*;

const SCORE_IF_DRAW: u32 = 3;
const SCORE_IF_WIN: u32 = 6;
const SCORE_IF_LOSE: u32 = 0;

fn main() {
    let input = fs::read_to_string("src/day2/input/input.txt")
        .expect("Cannot open input file");

    let games = input.trim().split('\n').map(|game| (game, evaluate_game(game)))
        .collect::<Vec<(&str, u32)>>();

    let total_score = games.iter().fold(0, |acc, game| acc + game.1);
    println!("Part one: Total score is: {}", total_score)
}

fn evaluate_game(game: &str) -> u32 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_score_is_calculated() {
        let score = evaluate_game("A Y");
        assert_eq!(score, 8);

        let score = evaluate_game("B X");
        assert_eq!(score, 1);

        let score = evaluate_game("C Z");
        assert_eq!(score, 6);
    }
}
