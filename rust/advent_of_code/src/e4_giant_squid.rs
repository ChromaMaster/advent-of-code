use std::fs;

pub fn execute() {
    let contents = fs::read_to_string("../inputs/e4_giant_squid_input.txt").expect("Cannot read the file");

    let numbers = get_bingo_numbers(contents.clone());
    let mut boards = get_bingo_boards(contents);

    println!("--- Part One ---");
    'outer: for number in numbers {
        for (index, board) in boards.iter_mut().enumerate() {
            board.mark_number(number);
            if board.is_winner() {
                let sum: u32 = board.get_not_marked_numbers().iter().map(|n| n.value).sum();

                println!("Winner is board {}!", index);
                println!("Result is: {}", sum * number.value);
                break 'outer;
            }
        }
    }


}

#[derive(Debug, Clone, Copy)]
struct Number {
    value: u32,
    marked: bool,
}

impl Number {
    pub fn new(number: String) -> Number {
        Number {
            value: number.parse().unwrap(),
            marked: false,
        }
    }

    fn mark(&mut self) {
        self.marked = true;
    }

    fn is_marked(&self) -> bool {
        self.marked
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

#[derive(Debug)]
struct Board {
    numbers: [[Option<Number>; 5]; 5],

    rows_line_count: [u32; 5],
    cols_line_count: [u32; 5],
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        for row in 0..self.numbers.len() {
            for col in 0..self.numbers[row].len() {
                if self.numbers[row][col].unwrap().value != other.numbers[row][col].unwrap().value {
                    return false;
                }
            }
        }
        true
    }
}

impl Board {
    pub fn new(string_board: &String) -> Board {
        let mut board = Board {
            numbers: [[None; 5]; 5],
            rows_line_count: [0u32; 5],
            cols_line_count: [0u32; 5],
        };

        for (row, line) in string_board.lines().enumerate() {
            for (col, number) in line.split_whitespace().enumerate() {
                // println!("{:?}:{:?} -> {}",row,col, number);
                board.numbers[row][col] = Some(Number::new(String::from(number)));
            }
        }

        board
    }

    fn mark_number(&mut self, number: Number) {
        for (r, row) in self.numbers.iter_mut().enumerate() {
            for (c, n) in row.iter_mut().enumerate() {
                if n.unwrap() == number {
                    n.as_mut().unwrap().mark();
                    self.rows_line_count[r] += 1;
                    self.cols_line_count[c] += 1;
                }
            }
        }
    }

    fn get_number(&self, row: usize, col: usize) -> Number {
        self.numbers[row - 1][col - 1].unwrap()
    }

    fn get_not_marked_numbers(&self) -> Vec<Number> {
        let not_marked_numbers = self.numbers
            .iter()
            .flat_map(|r| r.iter())
            .filter(|e| !(e.unwrap()).is_marked())
            .map(|e| e.unwrap())
            .collect();
        not_marked_numbers
    }

    fn is_winner(&self) -> bool {
        for n in self.rows_line_count {
            if n == 5 {
                return true;
            }
        }

        for n in self.cols_line_count {
            if n == 5 {
                return true;
            }
        }

        false
    }
}

fn get_bingo_numbers(string_bingo: String) -> Vec<Number> {
    let lines: Vec<String> = string_bingo.lines().map(|s| s.to_string()).collect();
    let string_numbers = lines.get(0).unwrap();
    let numbers: Vec<Number> = string_numbers.split(',').map(|s| Number::new(s.to_string())).collect();
    numbers
}

fn get_bingo_boards(string_bingo: String) -> Vec<Board> {
    let mut lines: Vec<String> = string_bingo.lines().map(|s| s.to_string()).collect();
    lines.push(String::new());
    let string_board: &mut String = &mut String::new();
    let mut boards: Vec<Board> = Vec::new();
    for line in lines.iter().skip(2) {
        if line.is_empty() {
            boards.push(Board::new(&string_board.trim().to_string()));
            string_board.clear();
        }
        string_board.push_str(line);
        string_board.push('\n');
    }
    boards
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_number_can_be_created_from_string() {
        let number = Number::new(String::from("5"));
        assert_eq!(number.value, 5)
    }

    #[test]
    fn a_number_can_be_marked() {
        let mut number = Number::new(String::from("10"));
        number.mark();
        assert!(number.is_marked())
    }

    #[test]
    fn a_board_can_be_created_from_string() {
        let string_board = String::from("22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19
");
        let board = Board::new(&string_board);
        assert_eq!(board.get_number(3, 3).value, 14)
    }

    #[test]
    fn a_board_is_able_to_mark_a_number_if_exists() {
        let string_board = String::from("22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19
");

        let mut board = Board::new(&string_board);
        board.mark_number(Number::new(String::from("7")));
        assert!(board.get_number(3, 5).is_marked())
    }

    #[test]
    fn when_a_line_is_marked_the_board_is_considered_winner() {
        let string_board = String::from("22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19
");

        let mut board = Board::new(&string_board);
        vec!["22", "13", "17", "11", "0"]
            .iter()
            .for_each(|&s| board.mark_number(Number::new(String::from(s))));
        assert!(board.is_winner());
    }

    #[test]
    fn all_non_marked_numbers_from_the_board_are_correctly_retrieved() {
        let string_board = String::from("22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19
");

        let mut board = Board::new(&string_board);
        vec!["8", "2", "23", "4", "24"]
            .iter()
            .for_each(|&s| board.mark_number(Number::new(String::from(s))));

        let not_marked_numbers = board.get_not_marked_numbers();
        let expected_numbers: Vec<Number> = vec!["22", "13", "17", "11", "0", "21", "9", "14", "16", "7", "6", "10", "3", "18", "5", "1", "12", "20", "15", "19"]
            .iter()
            .map(|&s| Number::new(String::from(s)))
            .collect();
        assert_eq!(not_marked_numbers, expected_numbers);
    }

    #[test]
    fn bingo_numbers_are_retrieved_from_string_bingo() {
        let string_bingo = String::from("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7");

        let numbers = get_bingo_numbers(string_bingo);
        let expected_numbers: Vec<Number> = vec!["7", "4", "9", "5", "11", "17", "23", "2", "0", "14", "21", "24", "10", "16", "13", "6", "15", "25", "12", "22", "18", "20", "8", "19", "3", "26", "1"]
            .iter()
            .map(|s| Number::new(s.to_string()))
            .collect();
        assert_eq!(numbers, expected_numbers)
    }

    #[test]
    fn bingo_boards_are_retrieved_from_string_bingo() {
        let string_bingo = String::from("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7");

        let boards = get_bingo_boards(string_bingo);
        let mut expected_boards: Vec<Board> = Vec::new();
        expected_boards.push(Board::new(&String::from("22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19")));
        expected_boards.push(Board::new(&String::from("3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6")));
        expected_boards.push(Board::new(&String::from("14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7")));
        assert_eq!(boards, expected_boards)
    }
}


