pub mod day01;
pub mod day02;
pub mod day03;

use std::fs;

pub fn read_input_file(path: &str) -> Vec<String> {
    let full_path = format!("input/{}", path);

    let input = fs::read_to_string(full_path).expect("Cannot open input file");

    input.trim().lines().map(String::from).collect()
}

pub fn write_output_file(path: &str, data: &str) {
    let full_path = format!("output/{}", path);
    fs::write(full_path, data).expect("Cannot write output file");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_example_input_file_can_be_read() {
        let input = read_input_file("example.txt");
        let expected_input = vec!["This", "is a", "file", "example"];
        assert_eq!(input, expected_input)
    }

    #[test]
    fn the_example_output_file_can_be_written() {
        let result = "This is a file example";

        write_output_file("example.txt", result);

        let output_file_path = "output/example.txt";
        let output = fs::read_to_string(output_file_path).expect("Cannot open output file");
        assert_eq!(output, result);

        fs::remove_file(output_file_path).expect("Cannot remove output file");
    }
}
