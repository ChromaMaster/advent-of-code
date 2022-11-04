const EXAMPLE_INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return (self.x == other.x) && (self.y == other.y);
    }
}

struct Line {
    start_point: Point,
    end_point: Point,
    points: Vec<Point>,
}

impl Line {
    pub fn new(start_point: Point, end_point: Point) -> Self {
        let mut line = Self {
            start_point,
            end_point,
            points: Vec::new(),
        };

        for x in line.start_point.x..=line.end_point.x {
            for y in line.start_point.y..=line.end_point.y {
                line.points.push(Point { x, y })
            }
        }
        line.points.dedup();
z
        line
    }
}


pub fn execute() {
    println!("{}", EXAMPLE_INPUT);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_point_can_be_created_and_its_component_retrieved() {
        let point = Point { x: 6, y: 9 };
        assert_eq!(point.x, 6);
        assert_eq!(point.y, 9);
    }

    #[test]
    fn two_points_can_be_compared() {
        let point1 = Point { x: 6, y: 9 };
        let point2 = Point { x: 6, y: 9 };

        assert_eq!(point1, point2);
    }

    #[test]
    fn a_line_can_be_created_and_the_start_and_end_points_can_be_retrieved() {
        let line = Line::new(Point { x: 3, y: 0 }, Point { x: 3, y: 3 });
        assert_eq!(line.start_point, Point { x: 3, y: 0 });
        assert_eq!(line.end_point, Point { x: 3, y: 3 });
    }

    #[test]
    fn when_a_line_is_created_the_points_between_start_and_end_are_created() {
        let line = Line::new(Point { x: 3, y: 0 }, Point { x: 3, y: 3 });

        let expected_points = vec![Point { x: 3, y: 0 }, Point { x: 3, y: 1 }, Point { x: 3, y: 2 }, Point { x: 3, y: 3 }];
        assert!(expected_points.iter().all(|item| line.points.contains(item)));
    }
}