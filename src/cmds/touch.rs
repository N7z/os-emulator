use crate::kernel::{Kernel, Node};

pub fn run(kernel: &mut Kernel, args: &[&str]) {
    let Some(&name) = args.first() else {
        println!("touch: missing operand");
        return;
    };

    if kernel.child(kernel.cwd, name).is_some() {
        return;
    }

    kernel.add(kernel.cwd, name, Node::File { data: Vec::new() });
}
