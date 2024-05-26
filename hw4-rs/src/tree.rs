use std::{
    array,
    sync::atomic::{AtomicPtr, AtomicUsize, Ordering},
};

const KEY_SIZE: usize = 26;
const KEY_OFFSET: u8 = b'a';

struct Node {
    children: [AtomicPtr<Node>; KEY_SIZE],
    occurance: AtomicUsize,
}

impl Node {
    fn new(occurance: usize) -> Self {
        Node {
            children: array::from_fn(|_| AtomicPtr::new(std::ptr::null_mut())),
            occurance: AtomicUsize::new(occurance),
        }
    }
    fn new_ptr(occurance: usize) -> *mut Self {
        Box::into_raw(Box::new(Self::new(occurance)))
    }
    fn increase(&self) {
        self.occurance.fetch_add(1, Ordering::AcqRel);
    }
    fn increase_child(&self, key: u8) {
        let idx = (key - KEY_OFFSET) as usize;
        self.increase_child_by_idx(idx);
    }
    fn increase_child_by_idx(&self, idx: usize) {
        let create_node = Self::new_ptr(1);
        if let Err(x) = self.children[idx].compare_exchange(
            std::ptr::null_mut(),
            create_node,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            unsafe {
                drop(Box::from_raw(create_node));
                &*x
            }
            .increase();
        }
    }
    fn insert_child(&self, key: u8) -> &Node {
        let idx = (key - KEY_OFFSET) as usize;
        self.insert_child_by_idx(idx)
    }
    fn insert_child_by_idx(&self, idx: usize) -> &Node {
        let create_node = Self::new_ptr(0);
        let result = match self.children[idx].compare_exchange(
            std::ptr::null_mut(),
            create_node,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => create_node,
            Err(x) => unsafe {
                drop(Box::from_raw(create_node));
                x
            },
        };
        unsafe { &*result }
    }
    fn get_child(&self, key: u8) -> Option<&Node> {
        let idx = (key - KEY_OFFSET) as usize;
        let ptr = self.children[idx].load(Ordering::Acquire);
        unsafe { ptr.as_ref() }
    }
}

pub struct Tree(Node);

impl Tree {
    pub fn new() -> Self {
        Tree(Node::new(0))
    }
    pub fn insert(&self, key: &[u8]) {
        if let Some((last, rest)) = key.split_last() {
            let mut current = &self.0;
            for &k in rest {
                current = current.insert_child(k);
            }
            current.increase_child(*last);
        } else {
            self.0.increase();
        }
    }
    pub fn get(&self, key: &[u8]) -> Option<usize> {
        let mut current = &self.0;
        for &k in key {
            match current.get_child(k) {
                Some(x) => current = x,
                None => return None,
            }
        }
        Some(current.occurance.load(Ordering::Acquire))
    }
    pub unsafe fn release(&self) {
        let mut stack = vec![&self.0];
        while let Some(node) = stack.pop() {
            for child in node.children.iter() {
                if let Some(x) = child.load(Ordering::SeqCst).as_ref() {
                    stack.push(x);
                }
            }
            drop(Box::from_raw(node as *const Node as *mut Node));
        }
    }
    pub unsafe fn merge(&self, other: &Self) {
        let mut stack = vec![(&self.0, &other.0)];
        while let Some((node, other_node)) = stack.pop() {
            if other_node.occurance.load(Ordering::SeqCst) != 0 {
                node.increase();
            }
            for (idx, other_child) in other_node.children.iter().enumerate() {
                if let Some(x) = other_child.load(Ordering::Acquire).as_ref() {
                    stack.push((node.insert_child_by_idx(idx), x));
                }
            }
        }
    }
}
