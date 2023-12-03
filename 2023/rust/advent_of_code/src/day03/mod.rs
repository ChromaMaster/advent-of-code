pub fn run_a(input: &Vec<String>) -> String {
    let mut result = 0;

    let mut parts: Vec<Part> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    for (i, line) in input.iter().enumerate() {
        let parts_in_line = extract_parts_from_row(i, line);
        parts.extend(parts_in_line);

        let symbols_in_line = extract_symbols_from_row(i, line);
        symbols.extend(symbols_in_line);
    }

    for part in parts.iter() {
        for symbol in symbols.iter() {
            if part.is_adjacent_to_symbol(symbol) {
                result += part.value;
            }
        }
    }

    result.to_string()
}

pub fn run_b(input: &Vec<String>) -> String {
    let result = 0;

    result.to_string()
}

#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    fn x_distance_to(&self, other: Point) -> usize {
        self.x.abs_diff(other.x)
    }

    fn y_distance_to(&self, other: Point) -> usize {
        self.y.abs_diff(other.y)
    }

    fn is_adjacent_to(&self, other: Point) -> bool {
        self.x_distance_to(other) <= 1 && self.y_distance_to(other) <= 1
    }

    fn points_between(&self, other: Point) -> Vec<Point> {
        let mut points = Vec::new();

        if self.y == other.y {
            for x in self.x + 1..other.x {
                points.push(Point::new(x, self.y));
            }
        }

        points
    }
}

#[derive(Debug, Copy, Clone)]
struct Symbol {
    value: char,
    position: Point,
}

impl Symbol {
    fn new() -> Symbol {
        Symbol {
            value: ' ',
            position: Point::default(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Part {
    value: u32,
    start: Point,
    end: Point,
}

impl Part {
    fn new() -> Part {
        Part {
            value: 0,
            start: Point::default(),
            end: Point::default(),
        }
    }

    fn is_adjacent_to_symbol(&self, symbol: &Symbol) -> bool {
        if self.start.is_adjacent_to(symbol.position) || self.end.is_adjacent_to(symbol.position) {
            return true;
        }

        for point in self.start.points_between(self.end) {
            if point.is_adjacent_to(symbol.position) {
                return true;
            }
        }

        false
    }
}

fn extract_parts_from_row(row: usize, input: &str) -> Vec<Part> {
    let mut parts = Vec::new();

    let mut number = String::new();

    let mut part = Part::new();

    // Add a dot at the end of the input to make sure the last number is added to the parts
    let input = format!("{}.", input);

    for (i, c) in input.chars().enumerate() {
        if c.is_ascii_digit() {
            if number.is_empty() {
                part.start = Point::new(i, row);
            }
            number.push(c);
        } else if !number.is_empty() {
            part.end = Point::new(i - 1, row);
            part.value = number.parse::<u32>().unwrap();
            parts.push(part);
            number.clear();
        }
    }

    parts
}

fn extract_symbols_from_row(row: usize, input: &str) -> Vec<Symbol> {
    let mut symbols = Vec::new();

    let mut symbol = Symbol::new();

    input.chars().enumerate().for_each(|(i, c)| {
        if !c.is_ascii_digit() && c != '.' {
            symbol.value = c;
            symbol.position = Point::new(i, row);
            symbols.push(symbol);
        }
    });

    symbols
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input_file;

    #[test]
    fn given_an_input_line_it_identifies_all_the_parts() {
        let input = "467..114..";

        let parts = extract_parts_from_row(0, input);

        assert_eq!(parts.len(), 2);
    }

    #[test]
    fn given_an_input_line_it_returns_the_parts_with_their_value_and_position() {
        let input = "467....114";

        let parts = extract_parts_from_row(0, input);

        assert_eq!(parts[0].value, 467);
        assert_eq!(parts[0].start, Point::new(0, 0));
        assert_eq!(parts[0].end, Point::new(2, 0));

        assert_eq!(parts[1].value, 114);
        assert_eq!(parts[1].start, Point::new(7, 0));
        assert_eq!(parts[1].end, Point::new(9, 0));
    }

    #[test]
    fn given_an_input_line_it_identifies_all_the_symbols() {
        let input = "*......617";
        let symbols = extract_symbols_from_row(0, input);

        assert_eq!(symbols.len(), 1);
    }

    #[test]
    fn given_an_input_line_it_returns_the_symbols_with_their_position() {
        let input = "617*......";
        let symbols = extract_symbols_from_row(0, input);

        assert_eq!(symbols[0].value, '*');
        assert_eq!(symbols[0].position, Point::new(3, 0));
    }

    #[test]
    fn given_two_points_it_returns_the_x_distance_between_them() {
        let point_a = Point::new(1, 1);
        let point_b = Point::new(4, 5);

        assert_eq!(point_a.x_distance_to(point_b), 3);
    }

    #[test]
    fn given_two_points_it_returns_the_y_distance_between_them() {
        let point_a = Point::new(1, 1);
        let point_b = Point::new(4, 5);

        assert_eq!(point_a.y_distance_to(point_b), 4);
    }

    #[test]
    fn given_two_points_it_return_if_they_are_adjacent() {
        let point_a = Point::new(1, 1);
        let point_b = Point::new(4, 5);

        assert!(!point_a.is_adjacent_to(point_b));

        let point_a = Point::new(1, 1);
        let point_b = Point::new(1, 2);

        assert!(point_a.is_adjacent_to(point_b));

        let point_a = Point::new(1, 1);
        let point_b = Point::new(2, 2);

        assert!(point_a.is_adjacent_to(point_b));
    }

    #[test]
    fn given_two_points_on_the_same_row_it_returns_the_points_between_them() {
        let point_a = Point::new(1, 1);
        let point_b = Point::new(4, 1);

        let points = point_a.points_between(point_b);

        assert_eq!(points.len(), 2);
        assert_eq!(points[0], Point::new(2, 1));
        assert_eq!(points[1], Point::new(3, 1));
    }

    #[test]
    fn it_returns_if_a_part_is_adjacent_to_a_symbol() {
        let part = Part {
            value: 123,
            start: Point::new(6, 7),
            end: Point::new(8, 7),
        };

        let symbol = Symbol {
            value: '*',
            position: Point::new(5, 8),
        };

        assert!(part.is_adjacent_to_symbol(&symbol));
    }

    #[test]
    fn given_a_schematic_it_returns_the_sum_of_all_part_numbers() {
        let input = read_input_file("example03a.txt");

        let expected = 4361;
        assert_eq!(run_a(&input), expected.to_string());
    }
}
