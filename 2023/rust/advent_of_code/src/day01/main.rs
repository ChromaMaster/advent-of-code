use advent_of_code::day01;

fn main() {
    let input = advent_of_code::read_input_file("input01.txt");

    let result = day01::run(&input, false);
    println!("Result part one: {}", result);
    advent_of_code::write_output_file("output01a.txt", &result);

    let result = day01::run(&input, true);
    println!("Result part two: {}", result);
    advent_of_code::write_output_file("output01b.txt", &result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_true() {
        assert!(true)
    }
}
