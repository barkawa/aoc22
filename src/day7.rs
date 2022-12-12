use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
enum FSObjectKind {
    File,
    Directory,
}

use FSObjectKind::*;

#[derive(Debug)]
struct FSObject {
    kind: FSObjectKind,
    name: String,
    size: Option<usize>,
}

#[derive(Debug)]
struct Node {
    data: FSObject,
    parent: Option<*mut Node>,
    children: Vec<Node>,
}

impl Node {
    fn new(data: FSObject) -> Node {
        Node {
            data,
            parent: None,
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, data: FSObject) {
        let parent = self as *mut Self;
        self.children.push(Node {
            data,
            parent: Some(parent),
            children: Vec::new(),
        })
    }

    fn size(&mut self) -> usize {
        match self.data.size {
            Some(size) => size,
            None => {
                let size: usize = self.children.iter_mut().map(|n| n.size()).sum();
                self.data.size = Some(size);
                size
            }
        }
    }
}

fn traverse(node: &Node, total_size: &mut usize) {
    if node.data.kind == Directory {
        let size = node.data.size.unwrap();
        if size < 100_000 {
            *total_size += size;
        }
        for child in node.children.iter() {
            traverse(child, total_size);
        }
    }
}

pub fn day7() {
    unsafe {
        let input = super::get_input();

        let regex_dollar = Regex::new(r"^\$.*").unwrap();
        let regex_command = Regex::new(r"^\$ (cd (/|\.\.|\w+)|ls)$").unwrap();
        let regex_ls = Regex::new(r"(\w+) (\S+)").unwrap();

        let mut lines_iter = input.iter();

        let mut root_node = Node::new(FSObject {
            kind: Directory,
            name: "/".to_string(),
            size: None,
        });

        let mut current_node = &mut root_node as *mut Node;

        while let Some(line) = lines_iter.next() {
            if regex_dollar.is_match(line) {
                let captures = regex_command.captures(line).unwrap();

                if captures[1].starts_with("cd") {
                        if &captures[2] == ".." {
                            current_node = (*current_node).parent.unwrap();
                        } else if &captures[2] == "/" {
                            current_node = &mut root_node as *mut Node;
                        } else {
                            current_node = (*current_node)
                                .children
                                .iter_mut()
                                .find(|n| n.data.name == &captures[2])
                                .unwrap();
                        }
                } else {
                    continue;
                }
            } else {
                let captures = regex_ls.captures(line).unwrap();

                match &captures[1] {
                    "dir" => {
                        (*current_node).add_child(FSObject {
                            kind: Directory,
                            name: captures[2].to_string(),
                            size: None,
                        });
                    },
                    file_size => {
                        (*current_node).add_child(FSObject {
                            kind: File,
                            name: captures[2].to_string(),
                            size: Some(file_size.parse().unwrap()),
                        });
                    },
                    
                }
            }
        }

        let root_size = root_node.size();

        let mut total_size: usize = 0;
        traverse(&root_node, &mut total_size);

        dbg!(total_size);

        let free_space = 70000000 - root_size;

        dbg!(free_space);

    }
}
