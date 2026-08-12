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

        let mut parts = prompt.split_whitespace();

        let Some(name) = parts.next() else {
            continue;
        };

        let args: Vec<&str> = parts.collect();

        match command_registry::resolve(name) {
            Some(cmd) => (cmd.run)(&args),
            None => println!("Unknown command: {}", name),
        };
    }
}
