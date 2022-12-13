use anyhow::Result;
use std::{cell::RefCell, fs, rc::Rc};

use day07::{parser, Action, Entry};

const MAX_SIZE: i32 = 70_000_000;
const FREE_SIZE: i32 = 30_000_000;

struct Dir {
    name: String,
    files: Vec<(i32, String)>,
    dirs: Vec<Rc<RefCell<Dir>>>,
    parent: Option<Rc<RefCell<Dir>>>,
}

impl Dir {
    fn find_smallest(&self, needed: i32) -> i32 {
        let mut size = self.size();
        if size < needed {
            size = i32::MAX;
        }
        for dir in self.dirs.iter() {
            let s = dir.borrow().find_smallest(needed);
            if s > needed && s < size {
                size = s
            }
        }
        size
    }
    fn size(&self) -> i32 {
        self.file_size()
            + self
                .dirs
                .iter()
                .fold(0, |acc, dir| acc + dir.borrow().size())
    }
    fn file_size(&self) -> i32 {
        self.files.iter().fold(0, |acc, (size, _)| acc + size)
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let ast = parser::LogParser::new()
        .parse(&input)
        .map_err(|err| err.map_token(|tok| tok.to_string()))?;
    let root = Rc::new(RefCell::new(Dir {
        name: "/".to_string(),
        files: Vec::new(),
        dirs: Vec::new(),
        parent: None,
    }));
    let mut current: Rc<RefCell<Dir>> = root.clone();
    // Skip cd / as we setup the root dir already
    for action in ast.actions.iter().skip(1) {
        match action {
            Action::ChangeDir(name) => {
                let dir = if name == ".." {
                    if let Some(ref parent) = current.borrow().parent {
                        parent.clone()
                    } else {
                        panic!("change to non existant parent directory")
                    }
                } else {
                    if let Some(dir) = current
                        .borrow()
                        .dirs
                        .iter()
                        .find(|d| &d.borrow().name == name)
                    {
                        dir.clone()
                    } else {
                        panic!("change to non existant directory")
                    }
                };
                current = dir;
            }
            Action::List(entries) => {
                for entry in entries {
                    match entry {
                        Entry::Dir(name) => {
                            let parent = current.clone();
                            current.borrow_mut().dirs.push(Rc::new(RefCell::new(Dir {
                                name: name.to_owned(),
                                files: Vec::new(),
                                dirs: Vec::new(),
                                parent: Some(parent),
                            })))
                        }
                        Entry::File(size, name) => {
                            current.borrow_mut().files.push((*size, name.to_owned()))
                        }
                    }
                }
            }
        }
    }

    let needed = FREE_SIZE - (MAX_SIZE - root.borrow().size());

    let sum = root.borrow().find_smallest(needed);

    println!("Sum: {}", sum);
    Ok(())
}
