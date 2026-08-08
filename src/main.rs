use std::io::{self, Write};

fn main() {
    println!("╔══════════════════════╗");
    println!("║     OS Emulator      ║");
    println!("╚══════════════════════╝");

    loop {
        print!("os> ");
        io::stdout().flush().unwrap();

        let mut prompt: String = String::new();

        io::stdin()
            .read_line(&mut prompt)
            .expect("Something wrong happened");

        if prompt.trim() == "exit" {
            break;
        }

        println!("{prompt}");
    }
}
