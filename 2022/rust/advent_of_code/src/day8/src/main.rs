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
        visible |= self.trees[pos.0][0..pos.1].iter().all(|&a| a < self.tree_hight(pos));
        // Look right
        visible |= self.trees[pos.0][pos.1 + 1..self.trees[pos.0].len()].iter().all(|&a| a < self.tree_hight(pos));
        // Look up
        visible |= self.trees[0..pos.0].iter().map(|e| e[pos.1]).all(|a| a < self.tree_hight(pos));
        // Look down
        visible |= self.trees[pos.0 + 1..self.trees.len()].iter().map(|e| e[pos.1]).all(|a| a < self.tree_hight(pos));

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

    fn input() -> &'static str {
        r"30373
25512
65332
33549
35390"
    }
}
