use std::io::{self, Write};

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
            Some(cmd) => println!("The command used was {}", cmd.name),
            None => println!("Unknown command: {}", prompt.trim()),
        };

        if prompt.trim() == "exit" {
            println!("Bye!");
            break;
        }
    }
}
