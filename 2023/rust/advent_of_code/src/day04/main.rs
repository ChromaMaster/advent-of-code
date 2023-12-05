use advent_of_code::day04;

fn main() {
    let input = advent_of_code::read_input_file("input04.txt");

    let result = day04::run_a(&input);
    println!("Result part one: {}", result);
    advent_of_code::write_output_file("output04a.txt", &result);

    let result = day04::run_b(&input);
    println!("Result part two: {}", result);
    advent_of_code::write_output_file("output04b.txt", &result);
}
