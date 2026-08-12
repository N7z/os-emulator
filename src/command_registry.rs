use crate::cmds::cat;
use crate::cmds::cd;
use crate::cmds::clear;
use crate::cmds::help;
use crate::cmds::ls;
use crate::cmds::mkdir;
use crate::cmds::pwd;
use crate::cmds::shutdown;
use crate::cmds::touch;
use crate::kernel::Kernel;

pub struct Command {
    pub name: &'static str,
    pub description: &'static str,
    pub run: fn(&mut Kernel, &[&str]),
}

pub const COMMANDS: &[Command] = &[
    Command {
        name: "help",
        description: "shows all available commands",
        run: help::run,
    },
    Command {
        name: "shutdown",
        description: "shuts the system down",
        run: shutdown::run,
    },
    Command {
        name: "clear",
        description: "clears the terminal screen",
        run: clear::run,
    },
    Command {
        name: "pwd",
        description: "prints the current directory",
        run: pwd::run,
    },
    Command {
        name: "ls",
        description: "lists the current directory contents",
        run: ls::run,
    },
    Command {
        name: "cd",
        description: "changes the current directory",
        run: cd::run,
    },
    Command {
        name: "mkdir",
        description: "creates a directory",
        run: mkdir::run,
    },
    Command {
        name: "touch",
        description: "creates an empty file",
        run: touch::run,
    },
    Command {
        name: "cat",
        description: "prints a file contents",
        run: cat::run,
    },
];

pub fn resolve(input: &str) -> Option<&Command> {
    COMMANDS.iter().find(|&cmd| cmd.name == input)
}
