use std::cmp::{Eq, PartialEq, PartialOrd};
use std::fmt;

// 有权边
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Edge<T> {
    a: usize,
    b: usize,
    weight: T,
}

impl<T> fmt::Display for Edge<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{:.2}-{}", self.a, self.weight, self.b)
    }
}

impl<T> Edge<T>
where
    T: Copy,
{
    pub fn new(a: usize, b: usize, weight: T) -> Self {
        Self { a, b, weight }
    }

    pub fn v(&self) -> usize {
        self.a
    }

    pub fn w(&self) -> usize {
        self.b
    }

    pub fn weight(&self) -> T {
        self.weight
    }

    pub fn other(&self, x: usize) -> usize {
        assert!(x == self.a || x == self.b);
        if x == self.a {
            return self.b;
        } else {
            return self.a;
        }
    }
}
