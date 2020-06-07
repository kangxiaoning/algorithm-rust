use std::cmp::Ordering;
use std::fmt;

// 有权边
#[derive(Clone)]
pub struct Edge<T> {
    a: usize,
    b: usize,
    weight: T,
}

impl<T> PartialEq for Edge<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl<T> Eq for Edge<T> where T: PartialEq {}

impl<T> PartialOrd for Edge<T>
where
    T: PartialEq + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl<T> Ord for Edge<T>
where
    T: PartialEq + PartialOrd + Eq,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.partial_cmp(&other.weight).unwrap()
    }
}

impl<T> fmt::Display for Edge<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}: {:.2}", self.a, self.b, self.weight)
    }
}

impl<T> Edge<T>
where
    T: Clone,
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
        self.weight.clone()
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
