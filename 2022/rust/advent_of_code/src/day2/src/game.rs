use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
    Unknown(String),
}

impl From<char> for Hand {
    fn from(chr: char) -> Self {
        match chr {
            'A' | 'X' => Hand::Rock,
            'B' | 'Y' => Hand::Paper,
            'C' | 'Z' => Hand::Scissors,
            _ => Hand::Unknown(format!("{}", chr))
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Hand::Rock => {
                match other {
                    Hand::Rock => Some(Equal),
                    Hand::Paper => Some(Less),
                    Hand::Scissors => Some(Greater),
                    _ => None
                }
            }
            Hand::Paper => {
                match other {
                    Hand::Rock => Some(Greater),
                    Hand::Paper => Some(Equal),
                    Hand::Scissors => Some(Less),
                    _ => None
                }
            }
            Hand::Scissors => {
                match other {
                    Hand::Rock => Some(Less),
                    Hand::Paper => Some(Greater),
                    Hand::Scissors => Some(Equal),
                    _ => None
                }
            }
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hand_is_identified() {
        let hand = Hand::from('A');
        assert_eq!(hand, Hand::Rock);

        let hand = Hand::from('B');
        assert_eq!(hand, Hand::Paper);

        let hand = Hand::from('C');
        assert_eq!(hand, Hand::Scissors);

        let hand = Hand::from('X');
        assert_eq!(hand, Hand::Rock);

        let hand = Hand::from('Y');
        assert_eq!(hand, Hand::Paper);

        let hand = Hand::from('Z');
        assert_eq!(hand, Hand::Scissors);
    }

    #[test]
    fn game_rules_are_applied() {
        assert_eq!(Hand::Rock, Hand::Rock);
        assert!(Hand::Rock < Hand::Paper);
        assert!(Hand::Rock > Hand::Scissors);

        assert_eq!(Hand::Paper, Hand::Paper);
        assert!(Hand::Paper < Hand::Scissors);
        assert!(Hand::Paper > Hand::Rock);

        assert_eq!(Hand::Scissors, Hand::Scissors);
        assert!(Hand::Scissors < Hand::Rock);
        assert!(Hand::Scissors > Hand::Paper);
    }
}