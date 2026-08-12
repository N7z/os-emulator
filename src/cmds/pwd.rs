use crate::kernel::Kernel;

pub fn run(kernel: &mut Kernel, _args: &[&str]) {
    println!("{}", kernel.path_of(kernel.cwd));
}
