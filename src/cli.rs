
use std::io::{self, Write};

use crate::filesystem::navigation::FileSystem;
use crate::style::*;
use crate::commands::*;
use crate::filesystem::node::Node;

static PROMPT: &str = ">> ";

pub fn run() {

    let mut filesystem = FileSystem::new();
    
    loop {
        set_style(&Style::PROMPT);
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("read line failed");
        let cmd: Vec<&str> = input.trim().split_whitespace().collect();

        match cmd.first() {
            Some(&"exit") => {
                break;
            }
            Some(&"clear") => {
                if cmd.len() == 1 {
                    clear_screen();
                } else {
                    set_style(&Style::ERROR);
                    println!("too many arguments");
                }
            }
            Some(&"help") => {
                // help function already has cmd.len() handling
                help(cmd);
            }
            Some(&"pwd") => {
                if cmd.len() == 1 {
                    set_style(&Style::NORMAL);
                    println!("{}", filesystem.pwd());
                } else {
                    set_style(&Style::ERROR);
                    println!("too many arguments");
                }
            }
            Some(&"cd") => {
                if cmd.len() == 2 {
                    match cmd.get(1) {
                        Some(path) => {
                            if let Err(error) = filesystem.cd(path) {
                                set_style(&Style::ERROR);
                                println!("{}", error);
                            }
                        }
                        None => {
                            // unwrap unwraps the Result
                            filesystem.cd("/home/daniel").unwrap();
                        }
                    }
                } else {
                    set_style(&Style::ERROR);
                    println!("too many arguments");
                }
            }
            Some(&"ls") => {
                if cmd.len() == 1 {
                    match filesystem.ls() {
                        Some(nodes) => {
                            for node in nodes {
                                match node {
                                    Node::Directory {name, ..} => {
                                        set_style(&Style::DIRECTORY);
                                        println!("{name}");
                                    }
                                    Node::File {name, ..} => {
                                        set_style(&Style::FILE);
                                        println!("{name}");
                                    }
                                    Node::Message {name} => {
                                        set_style(&Style::MESSAGE);
                                        println!("{name}");
                                    }
                                }
                            }
                        }
                        None => {
                            set_style(&Style::ERROR);
                            println!("could not read directory");
                        }
                    }
                } else {
                    set_style(&Style::ERROR);
                    println!("too many arguments");
                }
            }
            Some(_) => {
                set_style(&Style::ERROR);
                println!("command '{}' not found", input.trim());
            }
            None => {
            }
        }
    }
}
