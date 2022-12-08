// #![deny(unused)]

mod inode;

use std::cell::RefCell;
use std::fs;
use std::ops::Deref;
use std::rc::Rc;
use crate::inode::Directory;

#[derive(PartialEq, Eq, Debug)]
enum Command {
    Cd(String),
    Ls,
    Unknown(String),
}

impl From<&str> for Command {
    fn from(input: &str) -> Self {
        let c = input.replace("$ ", "");
        let mut split = c.split(' ');

        match split.next().unwrap() {
            "cd" => Command::Cd(split.next().unwrap().to_string()),
            "ls" => Command::Ls,
            cmd => Command::Unknown(cmd.to_string())
        }
    }
}

struct Cli {
    root_dir: Rc<RefCell<Directory>>,
    current_dir: Option<Rc<RefCell<Directory>>>,
}

impl Cli {
    pub fn new() -> Self {
        Self {
            root_dir: Rc::new(RefCell::new(Directory::new("/"))),
            current_dir: None,
        }
    }

    pub fn parse_command(&mut self, input: &str) {
        let cmd = Command::from(input);

        match cmd {
            Command::Cd(target) => {
                if target == "/" {
                    self.current_dir = Some(Rc::clone(&self.root_dir))
                }

                // TODO: Check if the dir already exists, if not insert it
                //  In any case, update the current_dir

                // println!("{:?}", dir)
            }
            Command::Ls => {}
            Command::Unknown(_) => {}
        }
    }

    pub fn pwd(&self) -> String {
        if let Some(cur) = &self.current_dir {
            cur.borrow().name.to_string()
        } else {
            String::new()
        }
    }

    pub fn ls(&self) -> String {
        String::new()
    }
}

fn main() {
    let input = fs::read_to_string("src/day7/input/input.txt")
        .expect("Cannot open input file");

    let lines = input.split('\n').map(|line| line.to_string()).collect::<Vec<String>>();

    for (index, line) in lines.iter().enumerate() {
        println!("{}. {}", index, line)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cd_command_is_identified() {
        assert_eq!(Command::from("$ cd target"), Command::Cd(String::from("target")))
    }

    #[test]
    fn ls_command_is_identified() {
        assert_eq!(Command::from("$ ls"), Command::Ls)
    }

    #[test]
    fn cd_command_creates_the_dir_and_changes_the_fs_current_dir() {
        let mut cli = Cli::new();

        cli.parse_command("$ cd /");

        assert_eq!(cli.pwd(), "/");
        assert_eq!(cli.ls(), "");
    }
}
