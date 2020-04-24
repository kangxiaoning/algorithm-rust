use std::cmp::Ordering;
use std::mem;

type Tree<K, V> = Option<Box<Node<K, V>>>;

struct Node<K: Ord, V: Clone> {
    key: K,
    value: V,
    left: Tree<K, V>,
    right: Tree<K, V>,
}

impl<K: Ord, V: Clone> Node<K, V> {
    pub fn new(k: K, v: V) -> Tree<K, V> {
        Some(Box::new(Node {
            key: k,
            value: v,
            left: None,
            right: None,
        }))
    }
}

pub struct BST<K: Ord, V: Clone> {
    root: Tree<K, V>,
    pub count: u64,
}

impl<K: Ord, V: Clone> BST<K, V> {
    pub fn new() -> Self {
        BST {
            root: None,
            count: 0,
        }
    }

    pub fn size(&self) -> u64 {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn insert(&mut self, k: K, v: V) {
        self.count += 1;
        let root = mem::replace(&mut self.root, None);
        self.root = self.insert_node(root, k, v);
    }

    fn insert_node(&mut self, node: Tree<K, V>, k: K, v: V) -> Tree<K, V> {
        match node {
            Some(mut d) => {
                match k.cmp(&d.key) {
                    Ordering::Less => d.left = self.insert_node(d.left, k, v),
                    Ordering::Greater => d.right = self.insert_node(d.right, k, v),
                    Ordering::Equal => (),
                }
                Some(d)
            }
            _ => Node::new(k, v),
        }
    }

    pub fn contains(&self, k: K) -> bool {
        self.contains_node(&self.root, k)
    }

    fn contains_node(&self, node: &Tree<K, V>, k: K) -> bool {
        match node {
            Some(d) => match k.cmp(&d.key) {
                Ordering::Less => self.contains_node(&d.left, k),
                Ordering::Greater => self.contains_node(&d.right, k),
                Ordering::Equal => true,
            },
            None => false,
        }
    }

    pub fn search(&self, k: K) -> Option<V> {
        self.search_node(&self.root, k)
    }

    fn search_node(&self, node: &Tree<K, V>, k: K) -> Option<V> {
        match node {
            Some(d) => match k.cmp(&d.key) {
                Ordering::Less => self.search_node(&d.left, k),
                Ordering::Greater => self.search_node(&d.right, k),
                Ordering::Equal => Some(d.value.clone()),
            },
            None => None,
        }
    }
}

pub fn run() {
    let mut bst: BST<&str, u64> = BST::new();
    bst.insert("a", 1);
    bst.insert("b", 2);
    println!("size: {}", bst.size());
    println!("is empty: {}", bst.is_empty());
    println!("contains 'a': {}", bst.contains("a"));
    println!("contains 'c': {}", bst.contains("c"));
    println!("search 'a': {:?}", bst.search("a"));
    println!("search 'c': {:?}", bst.search("c"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_empty() {
        let bst: BST<&str, u64> = BST::new();
        assert_eq!(bst.is_empty(), true);
    }

    #[test]
    fn insert() {
        let mut bst = BST::new();
        bst.insert("a", 1);
        bst.insert("b", 2);
        assert_eq!(bst.is_empty(), false);
    }

    #[test]
    fn size() {
        let mut bst = BST::new();
        bst.insert("a", 1);
        bst.insert("b", 2);
        assert_eq!(bst.size(), 2);
    }

    #[test]
    fn contains() {
        let mut bst = BST::new();
        bst.insert("a", 1);
        assert_eq!(bst.contains("a"), true);
        assert_eq!(bst.contains("b"), false);
    }

    #[test]
    fn search() {
        let mut bst = BST::new();
        bst.insert("a", 1);
        assert_eq!(bst.search("a"), Some(1));
        assert_eq!(bst.search("b"), None);
    }
}