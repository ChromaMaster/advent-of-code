#![deny(unused)]

use std::fs;

struct Forest {
    trees: Vec<Vec<u32>>,
}

impl Forest {
    pub fn tree_hight(&self, pos: (usize, usize)) -> u32 {
        self.trees[pos.0][pos.1]
    }

    pub fn tree_is_visible(&self, pos: (usize, usize)) -> bool {
        if pos.0 == 0 || pos.0 == self.trees.len() - 1
            || pos.1 == 0 || pos.1 == self.trees[0].len() - 1 {
            return true;
        }

        let mut visible = false;

        // Look left
        visible |= self.get_trees_from_left(pos).iter().all(|&&a| a < self.tree_hight(pos));
        // Look right
        visible |= self.get_trees_from_right(pos).iter().all(|&&a| a < self.tree_hight(pos));
        // Look up
        visible |= self.get_trees_from_above(pos).iter().all(|&&a| a < self.tree_hight(pos));
        // Look down
        visible |= self.get_trees_from_below(pos).iter().all(|&&a| a < self.tree_hight(pos));

        visible
    }

    pub fn get_number_of_visible_trees(&self) -> u32 {
        let mut visible_trees = 0u32;
        for i in 0..self.trees.len() {
            for j in 0..self.trees[0].len() {
                if self.tree_is_visible((i, j)) {
                    visible_trees += 1;
                }
            }
        }
        visible_trees
    }

    pub fn get_visibility_from_tree(&self, pos: (usize, usize)) -> u32 {
        let mut visibility = 1u32;
        let mut count = 0u32;

        // Left
        let mut trees = self.get_trees_from_left(pos);
        trees.reverse();
        for &tree in trees {
            count += 1;
            if tree >= self.tree_hight(pos) { break; }
        }
        visibility *= count;
        count = 0;

        // Right
        trees = self.get_trees_from_right(pos);
        for &tree in trees {
            count += 1;
            if tree >= self.tree_hight(pos) { break; }
        }
        visibility *= count;
        count = 0;

        // Up
        trees = self.get_trees_from_above(pos);
        trees.reverse();
        for &tree in trees {
            count += 1;
            if tree >= self.tree_hight(pos) { break; }
        }
        visibility *= count;
        count = 0;

        // Down
        trees = self.get_trees_from_below(pos);
        // trees.reverse();
        for &tree in trees {
            count += 1;
            if tree >= self.tree_hight(pos) { break; }
        }
        visibility *= count;

        visibility
    }

    pub fn get_best_position_to_place_the_camp(&self) -> u32 {
        let mut max_visibility = 0u32;
        for i in 0..self.trees.len() {
            for j in 0..self.trees[0].len() {
                let visibility = self.get_visibility_from_tree((i, j));
                if visibility > max_visibility {
                    max_visibility = visibility
                }
            }
        }
        max_visibility
    }

    fn get_trees_from_left(&self, pos: (usize, usize)) -> Vec<&u32> {
        self.trees[pos.0][0..pos.1].iter().collect::<Vec<&u32>>()
    }

    fn get_trees_from_right(&self, pos: (usize, usize)) -> Vec<&u32> {
        self.trees[pos.0][pos.1 + 1..self.trees[pos.0].len()].iter().collect::<Vec<&u32>>()
    }

    fn get_trees_from_above(&self, pos: (usize, usize)) -> Vec<&u32> {
        self.trees[0..pos.0].iter().map(|e| &e[pos.1]).collect::<Vec<&u32>>()
    }

    fn get_trees_from_below(&self, pos: (usize, usize)) -> Vec<&u32> {
        self.trees[pos.0 + 1..self.trees.len()].iter().map(|e| &e[pos.1]).collect::<Vec<&u32>>()
    }
}


impl From<&str> for Forest {
    fn from(input: &str) -> Self {
        let lines = input.split('\n').collect::<Vec<&str>>();

        let mut rows = Vec::with_capacity(lines.len());

        for line in lines {
            let tree_hights = line.chars().collect::<Vec<char>>();
            let mut cols = Vec::with_capacity(tree_hights.len());

            for th in tree_hights {
                cols.push(th.to_digit(10).unwrap())
            }

            rows.push(cols)
        }

        Self {
            trees: rows
        }
    }
}

fn main() {
    let input = fs::read_to_string("src/day8/input/input.txt")
        .expect("Cannot open input file");

    let forest = Forest::from(input.trim());

    println!("Part one: The total number of visible trees in the forest is: {}", forest.get_number_of_visible_trees());
    println!("Part two: The highest scenic score is: {}", forest.get_best_position_to_place_the_camp());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forest_can_be_represented_as_a_matrix() {
        let forest = Forest::from(input());

        assert_eq!(forest.tree_hight((0, 0)), 3);
        assert_eq!(forest.tree_hight((2, 2)), 3);
        assert_eq!(forest.tree_hight((3, 3)), 4);
    }

    #[test]
    fn is_possible_to_know_if_a_tree_is_visible() {
        let forest = Forest::from(input());

        assert!(forest.tree_is_visible((0, 0)));
        assert!(forest.tree_is_visible((3, 0)));
        assert!(forest.tree_is_visible((4, 4)));
        assert!(forest.tree_is_visible((2, 4)));

        assert!(forest.tree_is_visible((1, 1)));
        assert!(forest.tree_is_visible((2, 3)));
    }

    #[test]
    fn total_number_of_visible_trees_can_be_obtained() {
        let forest = Forest::from(input());

        assert_eq!(forest.get_number_of_visible_trees(), 21);
    }

    #[test]
    fn visibility_from_a_tree_can_be_obtained() {
        let forest = Forest::from(input());

        assert_eq!(forest.get_visibility_from_tree((1, 2)), 4);
        assert_eq!(forest.get_visibility_from_tree((3, 2)), 8);
    }

    fn input() -> &'static str {
        r"30373
25512
65332
33549
35390"
    }
}
