use crate::kernel::Kernel;

pub fn run(_kernel: &mut Kernel, _args: &[&str]) {
    println!("Bye!");
    std::process::exit(0);
}
