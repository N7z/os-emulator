use crate::kernel::Kernel;

pub fn run(_kernel: &mut Kernel, _args: &[&str]) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("╔══════════════════════╗");
    println!("║     OS Emulator      ║");
    println!("╚══════════════════════╝");
}
