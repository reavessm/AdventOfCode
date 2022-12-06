use std::collections::{HashMap, LinkedList};

#[derive(Clone, Debug)]
pub struct Stack<T: Clone + std::fmt::Debug> {
    list: LinkedList<T>,
}

impl<T: Clone + std::fmt::Debug> std::fmt::Display for Stack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for l in self.list.iter() {
            s += format!("[{:?}] ", l).as_str();
        }

        write!(f, "{}", s)
    }
}

impl<T: Clone + std::fmt::Debug> Stack<T> {
    pub fn new() -> Stack<T> {
        Self {
            list: LinkedList::<T>::new(),
        }
    }

    pub fn pop(&mut self) -> T {
        self.list.pop_back().unwrap()
    }

    pub fn push(&mut self, t: T) {
        self.list.push_back(t)
    }

    pub fn peek(&self) -> Option<&T> {
        self.list.back()
    }

    pub fn mv(&mut self, other: &mut Stack<T>, count: usize) {
        for _ in 0..count {
            self.move_single(other);
        }
    }

    pub fn mv_retain(&mut self, other: &mut Stack<T>, count: usize) {
        let mut v = Vec::<T>::new();

        for _ in 0..count {
            v.push(self.pop());
        }

        for value in v.iter().rev() {
            other.push(value.clone());
        }
    }

    fn move_single(&mut self, other: &mut Stack<T>) {
        other.push(self.pop());
    }
}

pub fn init_hash_map<T: Clone + std::fmt::Debug>(count: usize) -> HashMap<usize, Box<Stack<T>>> {
    let mut h = HashMap::<usize, Box<Stack<T>>>::new();

    for i in 0..count {
        h.insert(i + 1, Box::from(Stack::<T>::new()));
    }

    return h;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut from = Stack::<&str>::new();
        let mut to = Stack::<&str>::new();

        from.push("foo");
        from.push("bar");

        to.push("baz");

        // Test that push/peek works
        let f: &'static str = from.peek().unwrap();
        assert_eq!(f, "bar");

        let t: &'static str = to.peek().unwrap();
        assert_eq!(t, "baz");

        // Test that mv works
        println!("Before:\n1: {from}\n2: {to}");
        from.mv(&mut to, 1);
        println!("After:\n1: {from}\n2: {to}");

        let f: &'static str = from.peek().unwrap();
        assert_eq!(f, "foo");

        let t: &'static str = to.peek().unwrap();
        assert_eq!(t, "bar");
    }
}
