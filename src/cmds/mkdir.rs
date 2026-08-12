use crate::kernel::{Kernel, Node};

pub fn run(kernel: &mut Kernel, args: &[&str]) {
    let Some(&name) = args.first() else {
        println!("mkdir: missing operand");
        return;
    };

    if kernel.child(kernel.cwd, name).is_some() {
        println!("mkdir: cannot create directory '{}': File exists", name);
        return;
    }

    kernel.add(
        kernel.cwd,
        name,
        Node::Dir {
            children: Vec::new(),
        },
    );
}
