pub fn run(input: &Vec<String>, rules: &GameRules) -> String {
    let mut result = 0;

    for game_input in input {
        let game = Game::new(game_input.to_string());
        if game.is_possible(rules) {
            result += game.id();
        }
    }

    result.to_string()
}

pub struct GameRules {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

struct Game {
    game: String,
}

impl Game {
    fn new(game: String) -> Game {
        Game { game }
    }

    fn id(&self) -> u32 {
        self.game
            .split(':')
            .next()
            .unwrap()
            .split(' ')
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap()
    }

    fn sets(&self) -> Vec<&str> {
        self.game
            .split(':')
            .nth(1)
            .unwrap()
            .split(';')
            .map(|s| s.trim())
            .collect()
    }

    fn is_possible(&self, rules: &GameRules) -> bool {
        let sets = self.sets();

        for set in sets {
            let green = get_number_of_green_cubes(set);
            let red = get_number_of_red_cubes(set);
            let blue = get_number_of_blue_cubes(set);

            if green > rules.green || red > rules.red || blue > rules.blue {
                return false;
            }
        }

        true
    }
}

fn get_number_of_green_cubes(set: &str) -> u32 {
    get_number_of_cubes(set, "green")
}

fn get_number_of_red_cubes(set: &str) -> u32 {
    get_number_of_cubes(set, "red")
}

fn get_number_of_blue_cubes(set: &str) -> u32 {
    get_number_of_cubes(set, "blue")
}

fn get_number_of_cubes(set: &str, color: &str) -> u32 {
    if !set.contains(color) {
        return 0;
    }

    set
        .split(',')
        .find(|s| s.trim().ends_with(color))
        .unwrap()
        .trim()
        .split(' ')
        .next()
        .unwrap()
        .parse::<u32>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::read_input_file;
    use super::*;

    #[test]
    fn it_gets_the_correct_game_id() {
        let input = String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let game = Game::new(input);

        let expected = 1;
        assert_eq!(game.id(), expected);
    }

    #[test]
    fn it_splits_every_game_into_sets_of_cubes() {
        let input = String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let game = Game::new(input);

        let expected: Vec<String> = vec!["3 blue, 4 red", "1 red, 2 green, 6 blue", "2 green"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        assert_eq!(game.sets(), expected);
    }


    #[test]
    fn given_a_set_of_cubes_it_gets_the_number_of_green_cubes() {
        let input = String::from("1 red, 2 green, 6 blue");

        let expected = 2;
        assert_eq!(get_number_of_green_cubes(&input), expected);
    }

    #[test]
    fn given_a_set_of_cubes_it_gets_the_number_of_red_cubes() {
        let input = String::from("1 red, 2 green, 6 blue");

        let expected = 1;
        assert_eq!(get_number_of_red_cubes(&input), expected);
    }

    #[test]
    fn given_a_set_of_cubes_it_gets_the_number_of_blue_cubes() {
        let input = String::from("1 red, 2 green, 6 blue");

        let expected = 6;
        assert_eq!(get_number_of_blue_cubes(&input), expected);
    }

    #[test]
    fn given_a_set_of_cubes_it_returns_zero_if_there_are_no_cubes_of_that_color() {
        let input = String::from("3 blue, 4 red");

        let expected = 0;
        assert_eq!(get_number_of_green_cubes(&input), expected);
    }

    #[test]
    fn given_some_game_rules_is_able_to_determine_if_the_game_is_possible() {
        let input = String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let game = Game::new(input);

        let rules = GameRules {
            red: 12,
            green: 13,
            blue: 14,
        };

        assert!(game.is_possible(&rules));
    }

    #[test]
    fn given_some_game_rules_is_able_to_determine_if_the_game_is_not_possible() {
        let input = String::from("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red");
        let game = Game::new(input);

        let rules = GameRules {
            red: 12,
            green: 13,
            blue: 14,
        };

        assert!(!game.is_possible(&rules));
    }

    #[test]
    fn given_some_games_and_game_rules_is_able_to_get_a_sum_of_all_possible_games_ids() {
        let input = read_input_file("example02a.txt");

        let rules = GameRules {
            red: 12,
            green: 13,
            blue: 14,
        };

        let expected = 8;
        assert_eq!(run(&input, &rules), expected.to_string());
    }
}
