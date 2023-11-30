use std::{
    cell::RefCell,
    collections::HashMap,
    ops::{Deref, DerefMut},
    rc::Rc,
};

pub enum NewNode {
    None,
    HasNode(HashMap<String, NewTreeNode>),
}

pub struct NewTreeNode {
    dirs: NewNode,
}

pub type Node = Rc<RefCell<Box<Tree>>>;

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
}

impl NodePointer {
    pub fn new(s: impl Into<String>) -> Self {
        Self {
            node: Rc::from(RefCell::from(Box::from(Tree::new(s.into())))),
        }
    }

    pub fn add_dir(&mut self, name: impl Into<String>) {
        let name = name.into();
        let new_node = NodePointer::new(name.clone());
        new_node.node.borrow_mut().parent = Some(self.clone());

        self.node.borrow_mut().dirs.insert(name.clone(), new_node);
    }

    pub fn add_file(&self, name: impl Into<String>, size: usize) {
        let name = name.into();
        self.node.borrow_mut().files.insert(name.clone(), size);
    }

    pub fn cd(&mut self, new_node: NodePointer) {
        self.node = new_node.node;
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

    //pub fn build_size(&self) -> &NodePointer {
    pub fn build_size(&self, starting: usize) {
        let mut size = starting;

        size += self
            .node
            .clone()
            .borrow()
            .files
            .iter()
            .map(|(_, size)| size)
            .sum::<usize>();

        size += self
            .node
            .clone()
            .borrow()
            .dirs
            .iter()
            .map(|(_, n)| n.node.borrow().size)
            .sum::<usize>();

        {
            let n = self.node.clone();
            let n = n.borrow();
            if n.parent.is_some() {
                n.parent.clone().unwrap().build_size(size);
            }
        }

        self.node.clone().borrow_mut().size = size;
    }

    pub fn parse(&self, s: impl Into<String>) -> &NodePointer {
        let root = self.clone();
        let mut cur = root.clone();

        for line in s.into().lines() {
            let mut tokens = line.split_whitespace();
            let first = tokens.nth(0);

            if first.is_none() {
                break;
            }

            let second = tokens.nth(0);

            if second.is_none() {
                break;
            }

            match first.unwrap() {
                "$" => match second.unwrap() {
                    "cd" => {
                        let third = tokens.nth(0);
                        if third.is_none() {
                            continue;
                        }
                        let third = third.unwrap();
                        if third == "/" {
                            cur = root.clone();
                        }

                        let d = cur.node.clone();
                        let d = d.as_ref().borrow();

                        if third == ".." {
                            if d.parent.is_none() {
                                continue;
                            }
                            cur = d.parent.clone().unwrap();
                        }
                        let d = &d.dirs;
                        let dir = d.get(third);
                        if dir.is_none() {
                            continue;
                        }
                        cur.cd(dir.unwrap().clone());
                    }
                    _ => {}
                },
                "dir" => cur.add_dir(second.unwrap()),
                num @ _ => {
                    let num = num.parse::<usize>();
                    if num.is_err() {
                        continue;
                    }
                    cur.add_file(second.unwrap(), num.unwrap());
                }
            }
        }

        let mut v = Vec::new();
        self.leaf_dirs(&mut v);
        for n in v.iter() {
            n.build_size(0);
        }
        self
    }

    pub fn dfs(&self) -> DFS {
        DFS {
            curr: self.clone(),
            next: self
                .node
                .clone()
                .borrow()
                .dirs
                .values()
                .map(|x| x.clone())
                .collect(),
        }
    }
}

impl Default for NodePointer {
    fn default() -> Self {
        Self {
            node: Rc::from(RefCell::from(Box::from(Tree::default()))),
        }
    }
}

impl Deref for NodePointer {
    type Target = Rc<RefCell<Box<Tree>>>;
    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl DerefMut for NodePointer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node
    }
}

pub struct DFS {
    curr: NodePointer,
    next: Vec<NodePointer>,
}

impl Iterator for DFS {
    type Item = NodePointer;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next.len() == 0 {
            return None;
        }
        let next = self.next.remove(0);
        self.curr = next.clone();

        let n = self.curr.node.borrow();
        let mut dirs: Vec<NodePointer> = n.dirs.values().map(|x| x.clone()).collect();
        self.next.reverse();
        self.next.append(&mut dirs);
        self.next.reverse();

        return Some(self.curr.clone());
    }
}
