use advent_of_code::day03;

fn main() {
    let input = advent_of_code::read_input_file("input03.txt");

    let result = day03::run_a(&input);
    println!("Result part one: {}", result);
    advent_of_code::write_output_file("output03a.txt", &result);

    let result = day03::run_b(&input);
    println!("Result part two: {}", result);
    advent_of_code::write_output_file("output03b.txt", &result);
}
