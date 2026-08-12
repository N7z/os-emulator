use std::io::{self, Write};

mod cmds;
mod command_registry;
mod kernel;

fn main() {
    let mut kernel = kernel::Kernel::new();

    println!("╔══════════════════════╗");
    println!("║     OS Emulator      ║");
    println!("╚══════════════════════╝");

    loop {
        print!("os> ");
        io::stdout().flush().unwrap();

        let mut prompt = String::new();

        let bytes = io::stdin()
            .read_line(&mut prompt)
            .expect("Something wrong happened");

        if bytes == 0 {
            println!();
            break;
        }

        let mut parts = prompt.split_whitespace();

        let Some(name) = parts.next() else {
            continue;
        };

        let args: Vec<&str> = parts.collect();

        match command_registry::resolve(name) {
            Some(cmd) => (cmd.run)(&mut kernel, &args),
            None => println!("Unknown command: {}", name),
        };
    }
}
