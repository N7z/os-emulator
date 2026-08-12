use crate::cmds::help;
use crate::cmds::shutdown;

pub struct Command {
    pub name: &'static str,
    pub description: &'static str,
    pub run: fn(&[&str]),
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
];

pub fn resolve(input: &str) -> Option<&Command> {
    COMMANDS.iter().find(|&cmd| cmd.name == input)
}
