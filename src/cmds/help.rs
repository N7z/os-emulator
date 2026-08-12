use crate::command_registry::COMMANDS;
use crate::kernel::Kernel;

pub fn run(_kernel: &mut Kernel, _args: &[&str]) {
    println!("The available commands are:");

    for cmd in COMMANDS.iter() {
        println!("- {} | {}", cmd.name, cmd.description);
    }
}
