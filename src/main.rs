use std::io::{self, Write};

mod cmds;
mod command_registry;

fn main() {
    println!("╔══════════════════════╗");
    println!("║     OS Emulator      ║");
    println!("╚══════════════════════╝");

    loop {
        print!("os> ");
        io::stdout().flush().unwrap();

        let mut prompt = String::new();

        io::stdin()
            .read_line(&mut prompt)
            .expect("Something wrong happened");

        match command_registry::resolve(prompt.trim()) {
            Some(cmd) => (cmd.run)(),
            None => println!("Unknown command: {}", prompt.trim()),
        };

        if prompt.trim() == "exit" {
            println!("Bye!");
            break;
        }
    }
}
