
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
                clear_screen();
            }
            Some(&"help") => {
                help(cmd);
            }
            Some(&"pwd") => {
                set_style(&Style::NORMAL);
                println!("{}", filesystem.pwd());
            }
            Some(&"cd") => {
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
            }
            Some(&"ls") => {
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
                        println!("coul not read directory");
                    }
                }
            }
            Some(_) => {
                set_style(&Style::ERROR);
                println!("idk what {} is supposed to mean", input.trim());
            }
            None => {
                print!(">");
            }
        }
    }
}
