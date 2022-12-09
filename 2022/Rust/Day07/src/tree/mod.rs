use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub type Node = Rc<RefCell<Tree>>;

#[derive(Debug, Clone)]
pub struct NodePointer {
    pub node: Node,
}

#[derive(Debug, Clone)]
pub struct Tree {
    pub name: String,
    pub size: usize,
    pub parent: Option<NodePointer>,
    pub files: HashMap<String, usize>,
    pub dirs: HashMap<String, NodePointer>,
}

impl Default for Tree {
    fn default() -> Self {
        Self {
            name: "/".to_string(),
            size: 0,
            parent: None,
            files: HashMap::new(),
            dirs: HashMap::new(),
        }
    }
}

impl Tree {
    pub fn new(s: impl Into<String>) -> Self {
        Self {
            name: s.into(),
            ..Default::default()
        }
    }

    pub fn add_dir(&mut self, name: impl Into<String>) {
        let name = name.into();
        self.dirs.insert(name.clone(), NodePointer::new(name));
    }

    pub fn add_file(&mut self, name: impl Into<String>, size: usize) {
        let name = name.into();
        self.files.insert(name.clone(), size);
    }
}

impl NodePointer {
    pub fn new(s: impl Into<String>) -> Self {
        Self {
            node: Rc::from(RefCell::from(Tree::new(s.into()))),
        }
    }

    pub fn cd(&self, new_node: NodePointer) {
        let mut old_node = self.node.borrow_mut();
        let nn = new_node.node.borrow_mut();
        *old_node = nn.clone();
    }

    pub fn leaf_dirs(&self, vec: &mut Vec<NodePointer>) {
        let dir = self.node.borrow();
        if dir.dirs.len() == 0 {
            vec.push(self.clone());
        }
        for (_, d) in dir.dirs.iter() {
            d.leaf_dirs(vec);
        }
    }

    pub fn build_size(&self) {
        let node: &Node = &self.node.clone();
        let mut t = node.borrow_mut();
        t.size = t.files.iter().map(|(_, size)| size).sum();
        println!("Size: {}", t.size);

        t.size += t
            .dirs
            .iter()
            .map(|(_, n)| n.node.borrow().size)
            .sum::<usize>();

        if t.parent.is_some() {
            t.clone().parent.unwrap().build_size();
        }
    }
}

impl Default for NodePointer {
    fn default() -> Self {
        Self {
            node: Rc::from(RefCell::from(Tree::default())),
        }
    }
}
