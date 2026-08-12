use crate::command_registry::COMMANDS;

pub fn run(_args: &[&str]) {
    println!("The available commands are:");

    for cmd in COMMANDS.iter() {
        println!("- {} | {}", cmd.name, cmd.description);
    }
}
