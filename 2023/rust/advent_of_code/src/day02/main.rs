use advent_of_code::day02;

fn main() {
    let input = advent_of_code::read_input_file("input02.txt");
    let game_rules = day02::GameRules {
        red: 12,
        green: 13,
        blue: 14,
    };

    let result = day02::run(&input, &game_rules);
    println!("Result part one: {}", result);
    advent_of_code::write_output_file("output02a.txt", &result);
}
