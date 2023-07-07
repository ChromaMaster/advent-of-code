// #![deny(unused)]

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct HeightMap {
    data: Vec<Vec<String>>,

    current_pos: (usize, usize),
    highest_pos: (usize, usize),
}

impl From<&str> for HeightMap {
    fn from(input: &str) -> Self {
        let data = input
            .lines()
            .map(|line| line.chars().map(|c| c.to_string()).collect())
            .collect::<Vec<Vec<String>>>();
        let current_pos = find_in_matrix(&data, "S");
        let highest_pos = find_in_matrix(&data, "E");
        HeightMap { data, current_pos, highest_pos }
    }
}

impl HeightMap {
    fn pos(&self, x: usize, y: usize) -> Option<&str> {
        self.data.get(y).and_then(|row| row.get(x)).map(|s| s.as_str())
    }

    fn mv(&mut self, dir: Direction) -> Result<(), ()> {
        match dir {
            Direction::Up => {
                if self.current_pos.1 == 0 {
                    return Err(());
                }
                self.update_current_pos("^");
                self.current_pos.1 -= 1;
            }
            Direction::Down => {
                if self.current_pos.1 == self.data.len() - 1 {
                    return Err(());
                }
                self.current_pos.1 += 1;
            }
            Direction::Left => {
                if self.current_pos.0 == 0 {
                    return Err(());
                }
                self.current_pos.0 -= 1;
            }
            Direction::Right => {
                if self.current_pos.0 == self.data[0].len() - 1 {
                    return Err(());
                }
                self.current_pos.0 += 1;
            }
        }
        Ok(())
    }

    fn update_current_pos(&mut self, elem: &str) {
        self.data[self.current_pos.1][self.current_pos.0] = elem.to_string();
    }

    fn print_map(&self) {
        for row in &self.data {
            for elem in row {
                print!("{}", elem);
            }
            println!();
        }
    }
}

fn find_in_matrix(matrix: &[Vec<String>], e: &str) -> (usize, usize) {
    for (i, row) in matrix.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if col == e {
                return (j, i);
            }
        }
    }
    (0, 0)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn element_can_be_found_in_matrix() {
        let matrix = vec![
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
            vec!["d".to_string(), "e".to_string(), "f".to_string()],
            vec!["g".to_string(), "h".to_string(), "i".to_string()],
        ];
        assert_eq!(find_in_matrix(&matrix, "e"), (1, 1));
    }

    #[test]
    fn heightmap_is_created() {
        let heightmap = HeightMap::from(input());

        assert_eq!(heightmap.current_pos, (0, 0));
        assert_eq!(heightmap.highest_pos, (5, 2));
    }

    #[test]
    fn current_position_can_be_moved_and_map_is_updated() {
        let mut heightmap = HeightMap::from(input());

        let current = heightmap.current_pos;
        assert!(heightmap.mv(Direction::Down).is_ok());

        assert_eq!(heightmap.current_pos, (0, 1));
        assert_eq!(heightmap.pos(current.0, current.1), Some("v"));
    }

    fn input() -> &'static str {
        r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"
    }
}
