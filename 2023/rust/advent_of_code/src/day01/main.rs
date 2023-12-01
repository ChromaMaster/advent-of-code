use advent_of_code::day01;

fn main() {
    let input = advent_of_code::read_input_file("input01.txt");
    let result = day01::run(input);
    println!("Result: {}", result);
    advent_of_code::write_output_file("output01.txt", &result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_true() {
        assert!(true)
    }
}
