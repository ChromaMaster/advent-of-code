use std::collections::HashSet;
use std::fs;

struct Datastream {
    data: String,

    window_start: usize,
    window_size: usize,
}

impl Datastream {
    pub fn new(data_stream: String) -> Self {
        Self {
            data: data_stream,
            window_start: 0,
            window_size: 4,
        }
    }

    pub fn get_window(&self) -> String {
        self.data[self.window_start..self.window_start + self.window_size].to_string()
    }
}

fn main() {
    let input = fs::read_to_string("src/day6/input/input.txt")
        .expect("Cannot open input file");

    let mut device = Datastream::new(input);

    for _ in 0..device.data.len() {
        if all_chars_are_different(device.get_window()) {
            break;
        }
        device.window_start += 1;
    }

    println!("Part one: First start marker after character {}", device.window_start + device.window_size);
}

fn all_chars_are_different(s: String) -> bool {
    s.chars().collect::<HashSet<char>>().len() == s.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATASTREAM_INPUT: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";

    #[test]
    fn window_with_substr_of_4_can_be_obtained() {
        let data_stream = Datastream::new(DATASTREAM_INPUT.to_string());

        assert_eq!(data_stream.get_window(), "nznr")
    }

    #[test]
    fn data_stream_window_can_be_moved() {
        let mut data_stream = Datastream::new(DATASTREAM_INPUT.to_string());

        data_stream.window_start += 4;

        assert_eq!(data_stream.get_window(), "nfrf");
    }

    #[test]
    fn string_with_all_diferent_values_is_identified() {
        let s = String::from("nznr");
        assert!(!all_chars_are_different(s));

        let s = String::from("abcd");
        assert!(all_chars_are_different(s));
    }
}
