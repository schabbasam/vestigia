
use crate::style::*;
use std::io::{self, Write};
use crate::game_progress::GameProgress;

pub struct Command {
    pub name: &'static str,
    pub description: &'static str,
    pub usage: &'static str
}

pub const COMMANDS: &[Command] = &[
    Command {
        name: "help",
        description: "show available commands and their usage",
        usage: "help [command]"
    },
    Command {
        name: "exit",
        description: "exit the program",
        usage: "exit"
    },
    Command {
        name: "clear",
        description: "clear the terminal screen",
        usage: "clear"
    },
    Command {
        name: "cd",
        description: "change directory, argument is optional",
        usage: "cd [directory]"
    },
    Command {
        name: "pwd",
        description: "print working directory",
        usage: "pwd"
    },
    Command {
        name: "ls",
        description: "list all files and folders in current working directory",
        usage: "ls"
    },
    Command {
        name: "cat",
        description: "output the content of a file",
        usage: "cat [file]"
    },
    Command {
        name: "login",
        description: "log into a user",
        usage: "login"
    }
];

pub fn login(progress: &mut GameProgress) {
    set_style(&Style::CREDENTIALS);
    
    print!("{:<10}", "username:");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();

    print!("{:<10}", "password:");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();
    let password = password.trim();
    
    if username == "daniel.natsuki" && password == "11490964" {
        progress.credentials_unlocked = true;
        set_style(&Style::NORMAL);
        println!("access granted!");
    } else {
        set_style(&Style::ERROR);
        println!("access denied!");
    }
}

pub fn help(cmd: Vec<&str>) {

    if cmd.len() == 1 {
        set_style(&Style::HEADING);
        println!("available commands:");

        for command in COMMANDS {
            set_style(&Style::CMD_NAME);
            print!("{:<12}", command.name);
            set_style(&Style::NORMAL);
            println!("{}", command.description);
        }
    } else if cmd.len() == 2 {
        if let Some(arg) = cmd.get(1) {
            if let Some(command) = COMMANDS.iter().find(|c| c.name == *arg) {

                set_style(&Style::HEADING);
                print!("{:<14}", "help: ");
                set_style(&Style::CMD_NAME);
                println!("{}", command.name);

                set_style(&Style::HEADING);
                print!("{:<14}", "description: ");
                set_style(&Style::NORMAL);
                println!("{}", command.description);

                set_style(&Style::HEADING);
                print!("{:<14}", "usage: ");
                set_style(&Style::CMD_NAME);
                println!("{}", command.usage);

            } else {
                set_style(&Style::ERROR);
                println!("error: command '{}' not found", arg);
            }
        }
    } else {
        set_style(&Style::ERROR);
        println!("wrong usage: expected 1 or 2 arguments, got {}", cmd.len());
    }
}
