pub enum Node {
    Dir { children: Vec<usize> },
    File { data: Vec<u8> },
}

pub struct Entry {
    pub name: String,
    pub parent: usize,
    pub node: Node,
}

pub struct Kernel {
    pub entries: Vec<Entry>,
    pub cwd: usize,
}

impl Kernel {
    pub fn new() -> Self {
        Kernel {
            entries: vec![Entry {
                name: String::from("/"),
                parent: 0,
                node: Node::Dir {
                    children: Vec::new(),
                },
            }],
            cwd: 0,
        }
    }

    pub fn child(&self, dir: usize, name: &str) -> Option<usize> {
        match &self.entries[dir].node {
            Node::Dir { children } => children
                .iter()
                .copied()
                .find(|&idx| self.entries[idx].name == name),
            Node::File { .. } => None,
        }
    }

    pub fn add(&mut self, parent: usize, name: &str, node: Node) -> usize {
        let idx = self.entries.len();

        self.entries.push(Entry {
            name: String::from(name),
            parent,
            node,
        });

        if let Node::Dir { children } = &mut self.entries[parent].node {
            children.push(idx);
        }

        idx
    }

    pub fn path_of(&self, mut idx: usize) -> String {
        let mut parts = Vec::new();

        while idx != 0 {
            parts.push(self.entries[idx].name.as_str());
            idx = self.entries[idx].parent;
        }

        if parts.is_empty() {
            return String::from("/");
        }

        let mut path = String::new();

        for part in parts.iter().rev() {
            path.push('/');
            path.push_str(part);
        }

        path
    }
}
