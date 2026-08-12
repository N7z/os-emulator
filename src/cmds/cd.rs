use crate::kernel::{Kernel, Node};

pub fn run(kernel: &mut Kernel, args: &[&str]) {
    let name = match args.first() {
        Some(&name) => name,
        None => "/",
    };

    if name == "/" {
        kernel.cwd = 0;
        return;
    }

    if name == "." {
        return;
    }

    if name == ".." {
        kernel.cwd = kernel.entries[kernel.cwd].parent;
        return;
    }

    let Some(idx) = kernel.child(kernel.cwd, name) else {
        println!("cd: {}: No such file or directory", name);
        return;
    };

    match kernel.entries[idx].node {
        Node::Dir { .. } => kernel.cwd = idx,
        Node::File { .. } => println!("cd: {}: Not a directory", name),
    }
}
