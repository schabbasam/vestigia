
use std::io::{self, Write};

use crate::filesystem::navigation::FileSystem;
use crate::style::*;
use crate::commands::*;
use crate::filesystem::node::Node;
use crate::game_progress::GameProgress;
use crate::typewriter::typewriter;

static PROMPT: &str = ">> ";

pub fn run() {
    let mut progress    = GameProgress::new();
    let mut filesystem  = FileSystem::new();
    
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
                if cmd.len() <= 2 {
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
            Some(&"cat") => {
                if cmd.len() == 2 {
                    match filesystem.cat(cmd[1]) {
                        Ok(content) => {
                            set_style(&Style::NORMAL);
                            typewriter(content, 80, [5, 0, 0], false);

                            if filesystem.is_facility_map(cmd[1]) {
                                set_style(&Style::HEADING);
                                println!("\nPress ENTER to continue...");

                                let mut input = String::new();
                                io::stdin().read_line(&mut input).unwrap();

                                println!("\n");
                                break;
                            }
                        }
                        Err(error) => {
                            set_style(&Style::ERROR);
                            println!("{}", error);
                        }
                    }
                } else {
                    set_style(&Style::ERROR);
                    println!("error: expected 1 argument, got {}", cmd.len()-1);
                }
            }
            Some(&"login") => {
                if cmd.len() == 1 {
                    login(&mut progress, &mut filesystem);
                } else {
                    set_style(&Style::ERROR);
                    println!("too many arguments");
                }
            }
            Some(&"unlock") => {
                if cmd.len() == 2 {
                    unlock(cmd[1], &mut progress, &mut filesystem);
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
