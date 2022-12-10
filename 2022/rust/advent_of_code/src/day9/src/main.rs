#![deny(unused)]

use std::collections::HashSet;
use std::fs;
use std::hash::Hash;

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

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
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

struct Knot {
    pos: Position,
    last_position: Position,
    visited_positions: HashSet<Position>,
}

impl Knot {
    pub fn new() -> Self {
        Self {
            pos: Position::new(),
            last_position: Position::new(),
            visited_positions: HashSet::from([Position::new()]),
        }
    }
}

struct Rope {
    knots: Vec<Knot>,
}

impl Rope {
    pub fn new(knots: usize) -> Self {
        let mut rope = Rope {
            knots: Vec::with_capacity(knots),
        };

        for _ in 0..knots {
            rope.knots.push(Knot::new())
        }

        rope
    }

    pub fn move_rope(&mut self, mv: Move) {
        match mv {
            Move::Up(y) => {
                for _ in 0..y {
                    for i in 0..self.knots.len() {
                        self.move_knot(i, Move::Up(1));
                    }
                }
            }
            Move::Right(x) => {
                for _ in 0..x {
                    for i in 0..self.knots.len() {
                        self.move_knot(i, Move::Right(1));
                    }
                }
            }
            Move::Down(y) => {
                for _ in 0..y {
                    for i in 0..self.knots.len() {
                        self.move_knot(i, Move::Down(1));
                    }
                }
            }
            Move::Left(x) => {
                for _ in 0..x {
                    for i in 0..self.knots.len() {
                        self.move_knot(i, Move::Left(1));
                    }
                }
            }
            _ => {}
        }
    }

    fn move_knot(&mut self, pos: usize, mv: Move) {
        // Head
        if pos == 0 {
            let knot = self.knots.get_mut(pos).unwrap();
            knot.last_position = Position { ..knot.pos };
            knot.pos.update(mv);
            return;
        }

        let mut knots = self.knots.iter_mut();
        let previous_knot = knots.nth(pos - 1).unwrap();
        let knot = knots.next().unwrap();

        // let previous_knot = self.knots.get(pos - 1).clone().unwrap();
        if knot.pos.distance_to(&previous_knot.pos) <= 1 {
            return;
        }

        knot.last_position = Position { ..knot.pos };

        // If the head was diagonal relative to the tail, take it's latest position
        if knot.pos.is_diagonal_to(&previous_knot.last_position) {
            knot.pos = Position { ..previous_knot.last_position };
        } else {
            knot.pos.update(mv);
        }

        knot.visited_positions.insert(Position { ..knot.pos });
    }

    pub fn get_tail(&self) -> &Knot {
        self.knots.last().unwrap()
    }
}

fn main() {
    let input = fs::read_to_string("src/day9/input/input.txt")
        .expect("Cannot open input file");

    let lines = input.trim().split('\n').collect::<Vec<&str>>();

    // Part one

    let mut rope = Rope::new(2);
    for line in lines {
        rope.move_rope(Move::from(line));
    }

    println!("Part one: The rope tail visited {} positions", rope.get_tail().visited_positions.len());
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
        let mut rope = Rope::new(2);
        // H (H over T over s)

        rope.move_rope(Move::Right(4));
        // s . . T H

        assert_eq!(rope.get_head().pos, Position { x: 4, y: 0 });
        assert_eq!(rope.get_tail().pos, Position { x: 3, y: 0 });

        rope.move_rope(Move::Up(1));
        // . . . . H
        // s . . T .

        assert_eq!(rope.get_head().pos, Position { x: 4, y: 1 });
        assert_eq!(rope.get_tail().pos, Position { x: 3, y: 0 });

        rope.move_rope(Move::Right(1));
        // . . . . T H
        // s . . . . .

        assert_eq!(rope.get_head().pos, Position { x: 5, y: 1 });
        assert_eq!(rope.get_tail().pos, Position { x: 4, y: 1 });

        rope.move_rope(Move::Left(1));
        // . . . . H . (H over T)
        // s . . . . .

        assert_eq!(rope.get_head().pos, Position { x: 4, y: 1 });
        assert_eq!(rope.get_tail().pos, Position { x: 4, y: 1 });

        rope.move_rope(Move::Left(4));
        // . . . . H . (H over T)
        // s . . . . .

        assert_eq!(rope.get_head().pos, Position { x: 0, y: 1 });
        assert_eq!(rope.get_tail().pos, Position { x: 1, y: 1 });
    }

    #[test]
    fn tail_visited_positions_can_be_obtained() {
        let mut rope = Rope::new(2);
        rope.move_rope(Move::from("R 4"));
        rope.move_rope(Move::from("U 4"));
        rope.move_rope(Move::from("L 3"));
        rope.move_rope(Move::from("D 1"));
        rope.move_rope(Move::from("R 4"));
        rope.move_rope(Move::from("D 1"));
        rope.move_rope(Move::from("L 5"));
        rope.move_rope(Move::from("R 2"));
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

        assert_eq!(rope.get_tail().visited_positions.len(), 13);
    }
}
