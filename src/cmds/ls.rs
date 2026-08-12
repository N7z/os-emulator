use crate::kernel::{Kernel, Node};

pub fn run(kernel: &mut Kernel, _args: &[&str]) {
    let Node::Dir { children } = &kernel.entries[kernel.cwd].node else {
        return;
    };

    for &idx in children {
        let entry = &kernel.entries[idx];

        match entry.node {
            Node::Dir { .. } => println!("{}/", entry.name),
            Node::File { .. } => println!("{}", entry.name),
        }
    }
}
