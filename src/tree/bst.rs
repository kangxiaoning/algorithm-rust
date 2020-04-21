use std::cmp::Ordering;

// Node
pub enum BST<K: Ord, V> {
    Node {
        key: K,
        value: V,
        left: Box<BST<K>>,
        right: Box<BST<K>>,
    },
    Empty,
}

impl<K: Ord, V> BST<K, V> {
    pub fn new() -> Self {
        BST::Empty
    }

    pub fn create(k: K, v: V) -> Self {
        BST::Node {
            key: k,
            value: v,
            left: Box::new(BST::Empty),
            right: Box::new(BST::Empty),
        }
    }

    pub fn insert(&mut self, k: K, v: V) {
        match self {
            BST::Node {
                ref key,
                ref value,
                ref mut left,
                ref mut right,
            } => match k.cmp(key) {
                Ordering::Less => left.insert(k, v),
                Ordering::Greater => right.insert(k, v),
                Ordering::Equal => return,
            },
            BST::Empty => {
                *self = BST::create(k, v);
            }
        }
    }
}
