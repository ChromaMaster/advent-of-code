use std::collections::HashSet;
use std::fs;

struct Datastream {
    data: String,

    start_of_packet_window_pos: usize,
    start_of_packet_window_size: usize,

    start_of_message_window_pos: usize,
    start_of_message_window_size: usize,
}

impl Datastream {
    pub fn new(data_stream: String) -> Self {
        Self {
            data: data_stream,
            start_of_packet_window_pos: 0,
            start_of_packet_window_size: 4,
            start_of_message_window_pos: 0,
            start_of_message_window_size: 14,
        }
    }

    pub fn get_start_of_packet_window(&self) -> String {
        self.data[self.start_of_packet_window_pos..self.start_of_packet_window_pos + self.start_of_packet_window_size].to_string()
    }

    pub fn get_start_of_message_window(&self) -> String {
        self.data[self.start_of_message_window_pos..self.start_of_message_window_pos + self.start_of_message_window_size].to_string()
    }
}

fn main() {
    let input = fs::read_to_string("src/day6/input/input.txt")
        .expect("Cannot open input file");

    let mut device = Datastream::new(input);

    // Part one
    for _ in 0..device.data.len() {
        if all_chars_are_different(device.get_start_of_packet_window()) {
            break;
        }
        device.start_of_packet_window_pos += 1;
    }

    println!("Part one: First start packet marker after character {}", device.start_of_packet_window_pos + device.start_of_packet_window_size);

    // Part two

    device.start_of_message_window_pos = device.start_of_packet_window_pos;

    for _ in device.start_of_message_window_pos..device.data.len() {
        if all_chars_are_different(device.get_start_of_message_window()) {
            break;
        }
        device.start_of_message_window_pos += 1;
    }

    println!("Part two: First start message marker after character {}", device.start_of_message_window_pos + device.start_of_message_window_size);
}

fn all_chars_are_different(s: String) -> bool {
    s.chars().collect::<HashSet<char>>().len() == s.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATASTREAM_INPUT: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";

    #[test]
    fn start_of_packet_window_can_be_obtained() {
        let data_stream = Datastream::new(DATASTREAM_INPUT.to_string());

        assert_eq!(data_stream.get_start_of_packet_window(), "nznr")
    }

    #[test]
    fn start_of_packet_window_can_be_moved() {
        let mut data_stream = Datastream::new(DATASTREAM_INPUT.to_string());

        data_stream.start_of_packet_window_pos += 4;

        assert_eq!(data_stream.get_start_of_packet_window(), "nfrf");
    }

    #[test]
    fn start_of_message_window_can_be_obtained() {
        let mut data_stream = Datastream::new(DATASTREAM_INPUT.to_string());

        assert_eq!(data_stream.get_start_of_message_window(), "nznrnfrfntjfmv");
    }

    #[test]
    fn string_with_all_diferent_values_is_identified() {
        let s = String::from("nznr");
        assert!(!all_chars_are_different(s));

        let s = String::from("abcd");
        assert!(all_chars_are_different(s));
    }
}
