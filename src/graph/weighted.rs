use crate::graph::Edge;
use std::fmt;

// 有权稠密图 - 邻接矩阵
pub struct DenseWeightedGraph<T> {
    // 节点数
    n: usize,
    m: usize,
    directed: bool,
    g: Vec<Vec<Option<Edge<T>>>>,
}

pub trait WeightedGraph<T> {
    fn new(n: usize, directed: bool) -> Self;
    fn v(&self) -> usize;
    fn e(&self) -> usize;
    fn add_edge(&mut self, v: usize, w: usize, weight: T);
    fn has_edge(&self, v: usize, w: usize) -> bool;
    fn adj(&self, v: usize) -> Vec<&Edge<T>>;
    fn show(&self);
}

impl<T> WeightedGraph<T> for DenseWeightedGraph<T>
where
    T: Copy + fmt::Display,
{
    fn new(n: usize, directed: bool) -> Self {
        // g初始化为n*n的布尔矩阵, g[i][j]为None, 表示没有任和边
        let g = vec![vec![None; n]; n];

        let m = 0;

        Self { n, m, directed, g }
    }

    // 返回节点个数
    fn v(&self) -> usize {
        self.n
    }

    // 返回边的个数
    fn e(&self) -> usize {
        self.m
    }

    fn add_edge(&mut self, v: usize, w: usize, weight: T) {
        assert!(v < self.n && w < self.n);

        // 如果从v到w已经有边, 删除这条边
        if self.has_edge(v, w) {
            self.g[v][w] = None;
            if v != w && !self.directed {
                self.g[w][v] = None;
            }
            self.m -= 1;
        }

        self.g[v][w] = Some(Edge::new(v, w, weight));
        if !self.directed {
            self.g[w][v] = Some(Edge::new(v, w, weight));
        }

        self.m += 1;
    }

    fn has_edge(&self, v: usize, w: usize) -> bool {
        assert!(v < self.n && w < self.n);
        match self.g[v][w] {
            Some(_) => true,
            None => false,
        }
    }

    fn adj(&self, v: usize) -> Vec<&Edge<T>> {
        let mut ret = Vec::new();

        for edge in self.g[v].iter() {
            match edge {
                Some(v) => ret.push(v),
                None => (),
            }
        }
        ret
    }

    fn show(&self) {
        for i in 0..self.n {
            for j in 0..self.n {
                match &self.g[i][j] {
                    Some(v) => print!("{}\t", v.weight()),
                    None => print!("None\t"),
                }
            }
            println!();
        }
    }
}

// 有权稀梳图 - 邻接表
pub struct SparseWeightedGraph<T> {
    n: usize,
    m: usize,
    directed: bool,
    g: Vec<Vec<Option<Edge<T>>>>,
}

impl<T> WeightedGraph<T> for SparseWeightedGraph<T>
where
    T: Copy + fmt::Display,
{
    fn new(n: usize, directed: bool) -> Self {
        // g初始化为n*n的布尔矩阵, g[i][j]为None, 表示没有任和边
        let g = vec![vec![None; n]; n];
        let m = 0;
        Self { n, m, directed, g }
    }

    // 返回节点个数
    fn v(&self) -> usize {
        self.n
    }

    // 返回边的个数
    fn e(&self) -> usize {
        self.m
    }

    fn add_edge(&mut self, v: usize, w: usize, weight: T) {
        assert!(v < self.n && w < self.n);
        self.g[v].push(Some(Edge::new(v, w, weight)));
        if v != w && !self.directed {
            self.g[w].push(Some(Edge::new(w, v, weight)));
        }
        self.m += 1
    }

    fn has_edge(&self, v: usize, w: usize) -> bool {
        assert!(v < self.n && w < self.n);

        for edge in self.g[v].iter() {
            if let Some(edge) = edge {
                if edge.other(v) == w {
                    return true;
                }
            };
        }
        return false;
    }

    fn adj(&self, v: usize) -> Vec<&Edge<T>> {
        let mut ret = Vec::new();
        for edge in self.g[v].iter() {
            match edge {
                Some(v) => ret.push(v),
                None => (),
            }
        }
        ret
    }

    fn show(&self) {
        for i in 0..self.n {
            print!("vertex {}: \t", i);
            for v in self.adj(i) {
                print!("{}\t", v);
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edge() {
        // float
        let a = Edge::new(0, 1, 1.1);
        let b = Edge::new(0, 1, 1.2);
        assert_eq!(a < b, true);
        assert_eq!(a <= b, true);
        assert_eq!(a > b, false);
        assert_eq!(a >= b, false);
        assert_eq!(a == b, false);
        assert_eq!(a != b, true);

        // string
        let a = Edge::new(0, 1, "abc");
        let b = Edge::new(0, 1, "def");
        assert_eq!(a < b, true);
        assert_eq!(a <= b, true);
        assert_eq!(a > b, false);
        assert_eq!(a >= b, false);
        assert_eq!(a == b, false);
        assert_eq!(a != b, true);

        // i32
        let a = Edge::new(0, 1, -3);
        let b = Edge::new(0, 1, 3);
        assert_eq!(a < b, true);
        assert_eq!(a <= b, true);
        assert_eq!(a > b, false);
        assert_eq!(a >= b, false);
        assert_eq!(a == b, false);
        assert_eq!(a != b, true);
    }

    #[test]
    fn dense_weighted_graph() {
        let n = 5;
        let mut g: DenseWeightedGraph<f32> = DenseWeightedGraph::new(n, false);
        g.add_edge(0, 1, 1.1);
        g.add_edge(0, 2, 1.0);
        g.add_edge(1, 2, 1.2);
        g.add_edge(1, 3, 1.3);
        g.add_edge(1, 4, 1.4);
        g.add_edge(2, 4, 1.4);

        assert_eq!(g.has_edge(0, 1), true);
        assert_eq!(g.has_edge(0, 3), false);
        assert_eq!(g.n, 5);
        assert_eq!(g.m, 6);
    }

    #[test]
    fn sparse_weighted_graph() {
        let n = 5;
        let mut g: SparseWeightedGraph<f32> = SparseWeightedGraph::new(n, false);
        g.add_edge(0, 1, 1.1);
        g.add_edge(0, 2, 1.0);
        g.add_edge(1, 2, 1.2);
        g.add_edge(1, 3, 1.3);
        g.add_edge(1, 4, 1.4);
        g.add_edge(2, 4, 1.4);

        assert_eq!(g.has_edge(0, 1), true);
        assert_eq!(g.has_edge(0, 3), false);
        assert_eq!(g.n, 5);
        assert_eq!(g.m, 6);
    }
}
