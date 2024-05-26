const CHAR_VARIANTS: usize = 26;

fn to_variant(mut c: char) -> usize {
    assert!(
        c.is_ascii_alphabetic(),
        "expected alphabetic character, got {}",
        c
    );
    c.make_ascii_lowercase();
    c as usize - 'a' as usize
}

#[derive(Clone)]
struct Node<V> {
    children: [Option<Box<Node<V>>>; CHAR_VARIANTS],
    value: Option<V>,
}

impl<V> Node<V> {
    fn new() -> Self {
        Node {
            children: Default::default(),
            value: None,
        }
    }
    fn get_or_insert(&mut self, c: char) -> &mut Node<V> {
        self.children[to_variant(c)]
            .get_or_insert_with(|| Box::new(Node::new()))
            .as_mut()
    }
    fn get_child(&self, c: char) -> Option<&Node<V>> {
        self.children[to_variant(c)]
            .as_ref()
            .map(|node| node.as_ref())
    }
    fn insert(&mut self, path: impl Iterator<Item = char>, value: V) {
        let mut node = self;
        for c in path {
            node = node.children[to_variant(c)]
                .get_or_insert_with(|| Box::new(Node::new()))
                .as_mut();
        }
        node.value = Some(value);
    }
}

impl Node<usize> {
    fn increase_if_exist(&mut self, other: &Self) {
        match other.value {
            Some(0) | None => {}
            _ => *self.value.get_or_insert(0) += 1,
        }
        for (idx, other_child) in other.children.iter().enumerate() {
            if other_child.is_none() {
                continue;
            }
            self.children[idx]
                .get_or_insert_with(|| Box::new(Node::new()))
                .as_mut()
                .increase_if_exist(other_child.as_ref().unwrap());
        }
    }
}

#[derive(Clone)]
pub struct Tree<V>(Box<Node<V>>);

impl<V> Tree<V> {
    pub fn new() -> Self {
        Tree(Box::new(Node::new()))
    }
    pub fn into_ptr(self) -> usize {
        Box::into_raw(self.0) as usize
    }
    pub fn wrapper(&mut self) -> TreeWrapper<V> {
        TreeWrapper(&mut *self.0)
    }
    pub unsafe fn from_ptr(ptr: usize) -> Self {
        Tree(Box::from(unsafe { Box::from_raw(ptr as *mut Node<V>) }))
    }
}

pub struct TreeWrapper<'a, V>(&'a mut Node<V>);

impl<'a, V> TreeWrapper<'a, V> {
    pub fn insert(&mut self, path: impl Iterator<Item = char>, value: V) {
        self.0.insert(path, value);
    }
    // pub fn children(&'a mut self) -> Vec<TreeWrapper<'a,V>> {
    //         self.0.children.iter_mut().filter_map(|child| child.as_mut().map(|x|TreeWrapper(&mut **x))).collect::<Vec<_>>()
    // }
    pub fn get(&self, path: impl Iterator<Item = char>) -> Option<&V> {
        let mut node = &(*self.0);
        for c in path {
            node = node.get_child(c)?;
        }
        node.value.as_ref()
    }
    pub fn get_mut(&mut self, path: impl Iterator<Item = char>) -> &mut V
    where
        V: Default,
    {
        let mut node = &mut (*self.0);
        for c in path {
            node = node.get_or_insert(c);
        }
        if node.value.is_none() {
            node.value = Some(V::default());
        }
        node.value.as_mut().unwrap()
    }
    pub unsafe fn from_ptr(ptr: usize) -> TreeWrapper<'static, V> {
        TreeWrapper(&mut *(ptr as *mut Node<V>))
    }
    pub fn clone(&self) -> Tree<V>
    where
        V: Clone,
    {
        Tree(Box::new(self.0.clone()))
    }
}

impl<'a> TreeWrapper<'a, usize> {
    pub fn increase_if_exist(&mut self, other: &Self) {
        self.0.increase_if_exist(&other.0);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tree() {
        let mut tree = Tree::new();
        tree.wrapper().insert("hello".chars(), "world");
        assert_eq!(tree.wrapper().get("hello".chars()), Some(&"world"));
        assert_eq!(tree.wrapper().get("world".chars()), None);
    }
    #[test]
    fn test_tree_case_insensitive() {
        let mut tree = Tree::new();
        tree.wrapper().insert("hello".chars(), "world");
        assert_eq!(tree.wrapper().get("HELLO".chars()), Some(&"world"));
    }
    #[test]
    fn test_tree_partial() {
        let mut tree = Tree::new();
        tree.wrapper().insert("hello".chars(), "world");
        assert_eq!(tree.wrapper().get("he".chars()), None);
    }
    #[test]
    fn test_tree_empty() {
        let mut tree: Tree<usize> = Tree::new();
        assert_eq!(tree.wrapper().get("hello".chars()), None);
    }
    #[test]
    fn test_tree_overwrite() {
        let mut tree = Tree::new();
        tree.wrapper().insert("hello".chars(), "world");
        tree.wrapper().insert("hello".chars(), "world2");
        assert_eq!(tree.wrapper().get("hello".chars()), Some(&"world2"));
    }
    #[test]
    fn test_increase_if_exist() {
        let mut tree1 = Tree::new();
        tree1.wrapper().insert("hello".chars(), 1);

        let mut tree2 = Tree::new();
        tree2.wrapper().insert("hello".chars(), 1010);

        let mut tree3 = Tree::new();
        tree3.wrapper().insert("hello".chars(), 1310);
        tree3.wrapper().insert("helloworld".chars(), 1010);

        tree1.wrapper().increase_if_exist(&tree2.wrapper());
        tree1.wrapper().increase_if_exist(&tree3.wrapper());
        assert_eq!(tree1.wrapper().get("hello".chars()), Some(&3));
    }
}
