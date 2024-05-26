use std::array;

const CHAR_VARIANTS: usize = 26;

fn to_variant(mut c: char) -> usize {
    debug_assert!(
        c.is_ascii_alphabetic(),
        "expected alphabetic character, got {}",
        c
    );
    c.make_ascii_lowercase();
    c as usize - 'a' as usize
}

#[derive(Clone)]
pub struct Node {
    value: usize,
    children: [usize; CHAR_VARIANTS],
}

impl Node {
    pub fn new() -> Self {
        Node {
            value: 0,
            children: [0; CHAR_VARIANTS],
        }
    }
}

#[derive(Clone)]
pub struct Tree(Vec<Node>);

impl Tree {
    pub fn new() -> Self {
        Tree(vec![Node::new()])
    }
    pub fn into_ptr(self) -> usize {
        Box::into_raw(Box::new(self)) as usize
    }
    pub unsafe fn from_ptr(ptr: usize) -> Box<Self> {
        Box::from_raw(ptr as *mut Self)
    }
    pub unsafe fn from_ptr_borrow(ptr: usize) -> &'static mut Self {
        &mut *(ptr as *mut Self)
    }
    fn get_or_insert(&mut self, root: usize, variant: usize) -> usize {
        let child = self.0[root].children[variant];
        match child {
            0 => {
                self.0.push(Node::new());
                let new_node = self.0.len() - 1;
                self.0[root].children[variant] = new_node;
                new_node
            }
            _ => child,
        }
    }
    pub fn increase(&mut self, mut path: impl Iterator<Item = char>) {
        let mut node = 0;
        for c in path {
            node = self.get_or_insert(node, to_variant(c));
        }
        self.0[node].value += 1;
    }
    pub fn get(&self, path: impl Iterator<Item = char>) -> Option<usize> {
        let mut node = 0;
        for c in path {
            node = self.0[node].children[to_variant(c)];
            if node == 0 {
                return None;
            }
        }
        Some(self.0[node].value)
    }
    pub fn increase_if_exists(&mut self, other: &Self) {
        self.merge(0, other, 0);
    }
    fn merge(&mut self, root: usize, other: &Self, other_root: usize) {
        for idx in 0..CHAR_VARIANTS {
            let other_new_root = other.0[other_root].children[idx];
            if other_new_root == 0 {
                continue;
            }
            let self_new_root = self.get_or_insert(root, idx);
            self.0[self_new_root].value += 1;
            self.merge(self_new_root, other, other_new_root);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree() {
        let mut tree = Tree::new();
        tree.increase("abc".chars());
        tree.increase("abc".chars());
        tree.increase("abc".chars());
        tree.increase("abc".chars());
        assert_eq!(Some(4), tree.get("abc".chars()));
    }
    #[test]
    fn merge() {
        let mut tree = Tree::new();
        tree.increase("abc".chars());
        tree.increase("abc".chars());
        tree.increase("abc".chars());
        tree.increase("abc".chars());
        let mut other = Tree::new();
        other.increase("abc".chars());
        other.increase("abc".chars());
        other.increase("abc".chars());
        other.increase("abc".chars());
        tree.merge(0, &other, 0);
        assert_eq!(Some(5), tree.get("abc".chars()));
    }
    #[test]
    fn branch() {
        let mut tree = Tree::new();
        tree.increase("abc".chars());
        tree.increase("abc".chars());
        tree.increase("abc".chars());
        tree.increase("abc".chars());
        tree.increase("abcd".chars());
        tree.increase("abcd".chars());
        tree.increase("abcd".chars());
        tree.increase("abcd".chars());
        assert_eq!(Some(4), tree.get("abc".chars()));
        assert_eq!(Some(4), tree.get("abcd".chars()));
    }
}
