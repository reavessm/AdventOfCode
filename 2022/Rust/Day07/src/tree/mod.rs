use std::{
    cell::RefCell,
    collections::HashMap,
    ops::{Deref, DerefMut},
    rc::Rc,
};

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

    /*
    pub fn add_dir(&mut self, name: impl Into<String>) {
        let name = name.into();
        //println!("AddDir Before: {}", self.dirs.len());
        let new_node = NodePointer::new(name);
        new_node.node.borrow_mut().parent = Some(self);
        self.dirs.insert(name.clone(), new_node);
        //println!("AddDir After: {}", self.dirs.len());
    }
    */

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

    pub fn add_dir(&mut self, name: impl Into<String>) {
        let name = name.into();
        let new_node = NodePointer::new(name.clone());
        new_node.node.borrow_mut().parent = Some(self.clone());

        //self.node.borrow_mut().dirs.insert(name.clone(), new_node);
        //let node = self.node.clone();
        //node.borrow_mut().dirs.insert(name.clone(), new_node);
        //let dirs = &node.as_ref().borrow().dirs;
        //let mut dirs = dirs.clone();
        //dirs.insert(name.clone(), new_node);
        //node.as_ref().borrow().dirs.insert(name.clone(), new_node);

        self.node.borrow_mut().dirs.insert(name.clone(), new_node);
    }

    pub fn cd(&mut self, new_node: NodePointer) {
        //let mut old_node = &self.node;
        //let mut old_node = self.node.borrow_mut();
        //let mut old_node = old_node.borrow_mut();
        //let nn = new_node.node.borrow_mut();
        //*old_node = nn.clone();
        //old_node = &new_node.node;

        self.node = new_node.node;
    }

    pub fn leaf_dirs(&self, vec: &mut Vec<NodePointer>) {
        //let node = self.node.clone();
        //let dir = node.borrow_mut();
        //let dir = self.node.borrow();
        let dir = self.node.as_ref().borrow();
        if dir.dirs.len() == 0 {
            vec.push(self.clone());
        }
        for (_, d) in dir.dirs.iter() {
            d.leaf_dirs(vec);
        }
    }

    pub fn build_size(&self) -> &NodePointer {
        let mut v = Vec::new();
        self.leaf_dirs(&mut v);
        println!("Number of leaves: {}", v.len());

        for leaf in v.iter() {
            let node: &Node = &leaf.node.clone();
            let mut t = node.borrow_mut();
            t.size = t.files.iter().map(|(_, size)| size).sum();
            println!("Building Size for {}: {}", t.name, t.size);

            t.size += t
                .dirs
                .iter()
                .map(|(_, n)| n.node.borrow().size)
                .sum::<usize>();
            println!("Building Size: {}", t.size);

            if t.parent.is_some() {
                println!("Found parent");
                t.clone().parent.unwrap().build_size();
            } else {
                println!("Did not find parent");
            }
        }

        self
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
                            //cur = cur.node.clone().as_ref().borrow_mut().parent.unwrap();
                            if d.parent.is_none() {
                                println!("No parent");
                                continue;
                            }
                            cur = d.parent.clone().unwrap();
                        }
                        //println!("CD into {}", third);
                        let d = &d.dirs;
                        let dir = d.get(third);
                        if dir.is_none() {
                            continue;
                        }
                        cur.cd(dir.unwrap().clone());
                        //cur.cd(NodePointer::new(third));
                    }
                    _ => {
                        //println!("Nothing todo");
                    }
                },
                "dir" => {
                    //println!("Adding dir {}", second.unwrap());
                    /*
                    let third = tokens.nth(0);
                    if third.is_none() {
                        continue;
                    }
                    */
                    //let mut t = cur.node.as_ref().borrow_mut();
                    //t.add_dir(second.unwrap());
                    cur.add_dir(second.unwrap())
                }
                num @ _ => {
                    let num = num.parse::<usize>();
                    if num.is_err() {
                        continue;
                    }
                    let num = num.unwrap();
                    let mut tree = cur.node.as_ref().borrow_mut();
                    tree.add_file(second.unwrap(), num)
                }
            }
        }

        self.build_size()
    }

    pub fn dfs(&self) -> DFS {
        println!("DFS");
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
            node: Rc::from(RefCell::from(Tree::default())),
        }
    }
}

impl Deref for NodePointer {
    type Target = Rc<RefCell<Tree>>;
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
        println!("Next");
        if self.next.len() == 0 {
            return None;
        }
        let next = self.next.remove(0);
        /*
        if next.is_some() {
            self.curr = next.unwrap().clone();
        }
        */
        self.curr = next.clone();

        let n = self.curr.node.borrow();
        let mut dirs: Vec<NodePointer> = n.dirs.values().map(|x| x.clone()).collect();
        //dirs.append(&mut self.next.clone());
        //self.next = dirs;
        self.next.reverse();
        self.next.append(&mut dirs);
        self.next.reverse();

        return Some(self.curr.clone());
    }
}
