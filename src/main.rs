use std::io;

#[derive(Debug)]
struct BSTNode {
    value: i32,
    left: Option<Box<BSTNode>>,
    right: Option<Box<BSTNode>>
}

impl BSTNode {
    fn insert(&mut self, val: i32) {
        if val > self.value {
            match self.left {
                None => self.left = Some(Box::new(BSTNode {value:val, left:None, right:None})),
                Some(ref mut node) => node.insert(val)
            }
        }
        else {
            match self.right {
                None => self.right = Some(Box::new(BSTNode {value:val, left:None, right:None})),
                Some(ref mut node) => node.insert(val)
            }
        }
    }

    fn find(&mut self, val: i32) -> bool {
        if val == self.value {
            return true;
        }
        if val > self.value {
            match self.left {
                None => false,
                Some(ref mut node) => node.find(val)
            }
        }
        else {
            match self.right {
                None => false,
                Some(ref mut node) => node.find(val)
            }
        }
    }
}

fn main() {
    println!("Welcome to my bst!");
    let mut root: Option<BSTNode> = None;

    loop {
        println!("Please enter insert or find followed by a value, or input 'exit'");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read a line");

        let items = input.split_whitespace();
        let cmds: Vec<&str> = items.collect();

        let command = cmds[0];
        let val: i32 = cmds[1].trim().parse()
            .expect("Please enter a number!!");

        match command {
            command if command == "insert" => {
                match root {
                    None => root = Some(BSTNode {value:val, left:None, right:None}),
                    Some(ref mut node) => node.insert(val)
                }
            },
            command if command == "find" => {
                match root {
                    None => println!("Need a root to find"),
                    Some(ref mut node) => {
                        if node.find(val) {
                            println!("Yay, able to find {}", val);
                        }
                        else {
                            println!("did not find val");
                        }
                    }
                }
            },
            command if command == "exit" => break,
            _ => println!("Unrecoginzed command.")
        }
    }
}