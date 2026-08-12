use crate::kernel::{Kernel, Node};

pub fn run(kernel: &mut Kernel, args: &[&str]) {
    let Some(&name) = args.first() else {
        println!("cat: missing operand");
        return;
    };

    let Some(idx) = kernel.child(kernel.cwd, name) else {
        println!("cat: {}: No such file or directory", name);
        return;
    };

    match &kernel.entries[idx].node {
        Node::File { data } => print!("{}", String::from_utf8_lossy(data)),
        Node::Dir { .. } => println!("cat: {}: Is a directory", name),
    }
}
