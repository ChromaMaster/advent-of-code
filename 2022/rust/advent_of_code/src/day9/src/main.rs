// #![deny(unused)]

use std::collections::HashSet;
use std::fs;

#[derive(PartialEq, Debug)]
enum Move {
    Up(u32),
    Right(u32),
    Down(u32),
    Left(u32),
    Unknown(String),
}

impl From<&str> for Move {
    fn from(input: &str) -> Self {
        let mut split = input.split(' ');
        let direction = split.next().unwrap();
        let amount = split.next().unwrap().parse::<u32>().unwrap();

        match direction {
            "U" => Move::Up(amount),
            "R" => Move::Right(amount),
            "D" => Move::Down(amount),
            "L" => Move::Left(amount),
            _ => Move::Unknown(direction.to_string())
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn update(&mut self, mv: Move) {
        match mv {
            Move::Up(y) => self.y += y as i32,
            Move::Right(x) => self.x += x as i32,
            Move::Down(y) => self.y -= y as i32,
            Move::Left(x) => self.x -= x as i32,
            _ => {}
        }
    }

    pub fn is_diagonal_to(&self, other: &Position) -> bool {
        if self.x == other.x || self.y == other.y {
            return false;
        }

        (self.x - other.x).abs() == (self.y - other.y).abs()
    }

    pub fn distance_to(&self, other: &Position) -> u32 {
        (((other.x - self.x).pow(2) + (other.y - self.y).pow(2)) as f32).sqrt() as u32
    }
}

struct Bridge {
    head: Position,
    last_head_position: Position,
    tail: Position,
    tail_visited_positions: HashSet<Position>,
}

impl Bridge {
    pub fn new() -> Self {
        Self {
            head: Position::new(),
            last_head_position: Position::new(),
            tail: Position::new(),
            tail_visited_positions: HashSet::from([Position::new()]),
        }
    }

    pub fn follow_knots(&mut self, mv: Move) {
        match mv {
            Move::Up(y) => {
                for _ in 0..y {
                    self.move_head(Move::Up(1));
                    self.move_tail(Move::Up(1));
                }
            }
            Move::Right(x) => {
                for _ in 0..x {
                    self.move_head(Move::Right(1));
                    self.move_tail(Move::Right(1));
                }
            }
            Move::Down(y) => {
                for _ in 0..y {
                    self.move_head(Move::Down(1));
                    self.move_tail(Move::Down(1));
                }
            }
            Move::Left(x) => {
                for _ in 0..x {
                    self.move_head(Move::Left(1));
                    self.move_tail(Move::Left(1));
                }
            }
            _ => {}
        }
    }

    fn move_head(&mut self, mv: Move) {
        self.last_head_position = Position { ..self.head };
        self.head.update(mv);
    }

    fn move_tail(&mut self, mv: Move) {
        if self.tail.distance_to(&self.head) <= 1 {
            return;
        }

        // If the head was diagonal relative to the tail, take it's latest position
        if self.tail.is_diagonal_to(&self.last_head_position) {
            self.tail = Position { ..self.last_head_position };
        } else {
            self.tail.update(mv);
        }

        self.tail_visited_positions.insert(Position { ..self.tail });
    }
}

fn main() {
    let input = fs::read_to_string("src/day9/input/input.txt")
        .expect("Cannot open input file");

    let mut bridge = Bridge::new();

    let lines = input.trim().split('\n').collect::<Vec<&str>>();

    for line in lines {
        bridge.follow_knots(Move::from(line));
    }

    println!("Part one: The rope tail visited {} positions", bridge.tail_visited_positions.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn movement_is_correctly_identified() {
        let mv = Move::from("U 23");
        assert_eq!(mv, Move::Up(23));

        let mv = Move::from("D 16");
        assert_eq!(mv, Move::Down(16));

        let mv = Move::from("R 209");
        assert_eq!(mv, Move::Right(209));

        let mv = Move::from("L 1");
        assert_eq!(mv, Move::Left(1));
    }

    #[test]
    fn position_can_be_updated_using_a_move() {
        let mut pos = Position::new();

        pos.update(Move::Up(2));
        assert_eq!(pos, Position { x: 0, y: 2 });

        pos.update(Move::Right(4));
        assert_eq!(pos, Position { x: 4, y: 2 });

        pos.update(Move::Down(8));
        assert_eq!(pos, Position { x: 4, y: -6 });

        pos.update(Move::Left(2));
        assert_eq!(pos, Position { x: 2, y: -6 });
    }

    #[test]
    fn it_is_known_when_a_position_it_is_diagonal_to_other() {
        let pos = Position { x: 45, y: -34 };
        assert!(pos.is_diagonal_to(&Position { x: 44, y: -33 }));
        assert!(pos.is_diagonal_to(&Position { x: 50, y: -29 }));
        assert!(!pos.is_diagonal_to(&Position { x: 45, y: -33 }));
    }

    #[test]
    fn distance_between_positions_can_be_known() {
        let pos = Position::new();
        assert_eq!(pos.distance_to(&Position { x: 1, y: 0 }), 1);
        assert_eq!(pos.distance_to(&Position { x: 0, y: -1 }), 1);
        assert_eq!(pos.distance_to(&Position { x: 1, y: 1 }), 1);
        assert_eq!(pos.distance_to(&Position { x: 2, y: 2 }), 2);
    }

    #[test]
    fn tail_is_moved_when_head_is_moved_and_the_distance_remains_one() {
        let mut bridge = Bridge::new();
        // H (H over T over s)

        bridge.follow_knots(Move::Right(4));
        // s . . T H

        assert_eq!(bridge.head, Position { x: 4, y: 0 });
        // assert_eq!(bridge.tail, Position { x: 3, y: 0 });


        // bridge.follow_knots(Move::Up(1));
        // // . . . . H
        // // s . . T .

        // assert_eq!(bridge.head, Position { x: 4, y: 1 });
        assert_eq!(bridge.tail, Position { x: 3, y: 0 });
        //
        // bridge.follow_knots(Move::Right(1));
        // // . . . . T H
        // // s . . . . .

        assert_eq!(bridge.head, Position { x: 5, y: 1 });
        // assert_eq!(bridge.tail, Position { x: 4, y: 1 });
        //
        // bridge.follow_knots(Move::Left(1));
        // . . . . H . (H over T)
        // s . . . . .

        assert_eq!(bridge.head, Position { x: 4, y: 1 });
        assert_eq!(bridge.tail, Position { x: 4, y: 1 });

        bridge.follow_knots(Move::Left(4));
        // . . . . H . (H over T)
        // s . . . . .

        assert_eq!(bridge.head, Position { x: 0, y: 1 });
        assert_eq!(bridge.tail, Position { x: 1, y: 1 });
    }

    #[test]
    fn tail_visited_positions_can_be_obtained() {
        let mut bridge = Bridge::new();
        bridge.follow_knots(Move::from("R 4"));
        bridge.follow_knots(Move::from("U 4"));
        bridge.follow_knots(Move::from("L 3"));
        bridge.follow_knots(Move::from("D 1"));
        bridge.follow_knots(Move::from("R 4"));
        bridge.follow_knots(Move::from("D 1"));
        bridge.follow_knots(Move::from("L 5"));
        bridge.follow_knots(Move::from("R 2"));
        /*
        . . . . . .
        . . . . . .
        . T H . . .
        . . . . . .
        s . . . . .

        . . # # . .
        . . . # # .
        . # # # # .
        . . . . # .
        s # # # . .
        */

        assert_eq!(bridge.tail_visited_positions.len(), 13);
    }
}
