use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::fmt::{Debug};
use std::rc::Rc;

trait INode: Debug {
    fn size(&self) -> u32;
    fn as_any(&self) -> &dyn Any;
}

type Node = Box<dyn INode>;

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Debug)]
pub struct Directory {
    pub name: String,
    inodes: Vec<Node>,
}

impl Directory {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), inodes: vec![] }
    }
    pub fn add_file(&mut self, file: File) {
        self.inodes.push(Box::new(file));
    }
    pub fn add_directory(&mut self, dir: Directory) {
        self.inodes.push(Box::new(dir));
    }

    pub fn files(&self) -> Vec<&File> {
        self.inodes
            .iter()
            .filter(|inode| inode.as_any().type_id() == TypeId::of::<File>())
            .map(|inode| inode.as_any().downcast_ref::<File>().unwrap())
            .collect::<Vec<&File>>()
    }

    pub fn directories(&self) -> Vec<&Directory> {
        self.inodes
            .iter()
            .filter(|inode| inode.as_any().type_id() == TypeId::of::<Directory>())
            .map(|inode| inode.as_any().downcast_ref::<Directory>().unwrap())
            .collect::<Vec<&Directory>>()
    }
}

impl INode for Directory {
    fn size(&self) -> u32 {
        self.inodes.iter().fold(0, |acc, inode| acc + inode.size())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl PartialEq for Directory {
    fn eq(&self, other: &Self) -> bool {
        let mut equals = self.name == other.name;
        equals &= self.inodes.len() == other.inodes.len();
        equals &= self.inodes.iter().zip(other.inodes.iter()).filter(|&(a, b)| a != b).count() == 0;
        equals
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct File {
    name: String,
    size: u32,
}

impl File {
    pub fn new(name: &str, size: u32) -> Self {
        Self { name: name.to_string(), size }
    }
}

impl INode for File {
    fn size(&self) -> u32 {
        self.size
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directory_can_be_created() {
        let dir = Directory::new("/");
        assert_eq!(dir.name, "/");
        assert_eq!(dir.size(), 0);
    }

    #[test]
    fn file_can_be_created() {
        let file = File::new("foo", 35);

        assert_eq!(file.name, "foo");
        assert_eq!(file.size(), 35);
    }

    #[test]
    fn file_can_be_added_to_a_directory() {
        let mut dir = Directory::new("/");
        let file = File::new("foo", 35);

        dir.add_file(file);

        assert_eq!(dir.files(), vec![&File { name: String::from("foo"), size: 35 }])
    }

    #[test]
    fn directories_can_be_added_to_a_directory() {
        let mut root_dir = Directory::new("/");
        let dir = Directory::new("a");

        root_dir.add_directory(dir);

        assert_eq!(root_dir.directories(), vec![&Directory { name: String::from("a"), inodes: vec![] }])
    }

    #[test]
    fn size_of_file_can_be_obtained() {
        let file = File::new("foo", 35);

        assert_eq!(file.size(), 35);
    }

    #[test]
    fn size_of_directory_with_files_can_be_obtained() {
        let mut dir = Directory::new("/");
        let file = File::new("foo", 35);

        dir.add_file(file);

        assert_eq!(dir.size(), 35);

        let file2 = File::new("bar", 64);
        dir.add_file(file2);

        assert_eq!(dir.size(), 99);
    }

    #[test]
    fn size_of_directory_with_files_and_directories_can_be_obtained() {
        let mut dir = Directory::new("/");
        let file = File::new("foo", 35);
        dir.add_file(file);

        let file2 = File::new("bar", 64);
        dir.add_file(file2);

        let mut dir2 = Directory::new("a");

        let file3 = File::new("foobar", 33);
        dir2.add_file(file3);

        dir.add_directory(dir2);

        assert_eq!(dir.size(), 132);
    }
}