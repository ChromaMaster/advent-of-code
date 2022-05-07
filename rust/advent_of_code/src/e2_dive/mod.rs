use std::fs;

pub fn execute() {
    let contents = fs::read_to_string("../inputs/e2_dive_input.txt").expect("Cannot read the file");

    let c: Vec<&str> = contents.lines().collect();
    let commands = parse_string_commands(c.iter().map(|&s| s.to_string()).collect());

    println!("--- Part One ---");
    let mut submarine = Submarine { ..Default::default() };
    submarine.drive(commands);
    println!("There final position of the submarine is {} ", submarine.get_overall_position());
}

fn parse_string_commands(str_commands: Vec<String>) -> Vec<Command> {
    let mut commands = Vec::new();
    str_commands.iter().for_each(|command| {
        let c: Vec<&str> = command.split_whitespace().collect();
        let instr = c[0];
        let val = c[1].parse::<u32>().unwrap();

        match instr {
            "forward" => {
                commands.push(Command::Forward(val));
            }
            "down" => {
                commands.push(Command::Down(val));
            }
            "up" => {
                commands.push(Command::Up(val));
            }
            _ => {}
        }
    });
    return commands;
}

#[derive(Debug, PartialEq)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

#[derive(Default)]
struct Position {
    horizontal: u32,
    depth: u32,
}

#[derive(Default)]
struct Submarine {
    position: Position,
}

impl Submarine {
    fn drive(&mut self, commands: Vec<Command>) {
        for command in commands.iter() {
            self.parse_command(command)
        }
    }

    fn parse_command(&mut self, command: &Command) {
        match command {
            Command::Forward(val) => {
                self.position.horizontal += val
            }
            Command::Down(val) => {
                self.position.depth += val
            }
            Command::Up(val) => {
                self.position.depth -= val
            }
        }
    }

    fn get_overall_position(&self) -> u32 {
        return self.position.horizontal * self.position.depth;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_string_commands_into_commands_type_correctly() {
        let commands: Vec<String> = vec!["forward 5", "down 5", "forward 8", "up 3", "down 8", "forward 2"]
            .iter()
            .map(|&s| s.to_string())
            .collect();
        let parsed_commands = parse_string_commands(commands);
        assert_eq!(parsed_commands, vec![Command::Forward(5), Command::Down(5), Command::Forward(8), Command::Up(3), Command::Down(8), Command::Forward(2)])
    }

    #[test]
    fn it_detects_the_forward_command_and_increases_the_horizontal_position() {
        let mut submarine = Submarine { ..Default::default() };
        let command = Command::Forward(3);

        submarine.parse_command(&command);
        assert_eq!(submarine.position.horizontal, 3)
    }

    #[test]
    fn it_detects_the_down_command_and_increases_the_depth_position() {
        let mut submarine = Submarine { ..Default::default() };
        let command = Command::Down(2);

        submarine.parse_command(&command);
        assert_eq!(submarine.position.depth, 2)
    }

    #[test]
    fn it_detects_the_up_command_and_decreases_the_depth_position() {
        let mut submarine = Submarine { position: Position { depth: 5, ..Default::default() } };
        let command = Command::Up(2);

        submarine.parse_command(&command);
        assert_eq!(submarine.position.depth, 3)
    }

    #[test]
    fn it_returns_the_overall_position() {
        let submarine = Submarine { position: Position { horizontal: 15, depth: 10 } };
        assert_eq!(submarine.get_overall_position(), 150)
    }

    #[test]
    fn it_drives_the_submarine_correctly() {
        let mut submarine = Submarine { position: Position { horizontal: 0, depth: 0 } };
        submarine.drive(vec![Command::Forward(5), Command::Down(5), Command::Forward(8), Command::Up(3), Command::Down(8), Command::Forward(2)]);
        assert_eq!(submarine.get_overall_position(), 150)
    }
}