pub mod components;
pub mod path;
pub mod readgraph;

// use readgraph;

use rand::{self, Rng};
use std::cmp::{Eq, PartialEq, PartialOrd};
use std::fmt;
use std::iter::Iterator;
use std::path::Path;

pub trait Graph {
    fn new(n: usize, directed: bool) -> Self;
    fn v(&self) -> usize;
    fn e(&self) -> usize;
    fn add_edge(&mut self, v: usize, w: usize);
    fn has_edge(&self, v: usize, w: usize) -> bool;
    fn adj(&self, v: usize) -> Vec<usize>;
    fn show(&self);
}

// 稠密图 - 邻接矩阵
pub struct DenseGraph {
    n: usize,
    m: usize,
    directed: bool,
    g: Vec<Vec<bool>>,
}

impl Graph for DenseGraph {
    fn new(n: usize, directed: bool) -> Self {
        // g初始化为n*n的布尔矩阵, g[i][j]为false, 表示没有任和边
        let g = vec![vec![false; n]; n];
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

    fn add_edge(&mut self, v: usize, w: usize) {
        assert!(v < self.n && w < self.n);

        self.g[v][w] = true;
        if !self.directed {
            self.g[w][v] = true;
        }

        self.m += 1;
    }

    fn has_edge(&self, v: usize, w: usize) -> bool {
        assert!(v < self.n && w < self.n);
        self.g[v][w]
    }

    fn adj(&self, v: usize) -> Vec<usize> {
        let mut ret = Vec::new();

        for (idx, &is_true) in self.g[v].iter().enumerate() {
            if is_true {
                ret.push(idx);
            }
        }
        ret
    }

    fn show(&self) {
        for i in 0..self.n {
            for j in 0..self.n {
                print!("{}\t", self.g[i][j]);
            }
            println!();
        }
    }
}

// 稀梳图 - 邻接表
pub struct SparseGraph {
    n: usize,
    m: usize,
    directed: bool,
    g: Vec<Vec<usize>>,
}

impl Graph for SparseGraph {
    fn new(n: usize, directed: bool) -> Self {
        let g = vec![vec![]; n];
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

    fn add_edge(&mut self, v: usize, w: usize) {
        assert!(v < self.n && w < self.n);

        self.g[v].push(w);
        if v != w && !self.directed {
            self.g[w].push(v);
        }

        self.m += 1;
    }

    fn has_edge(&self, v: usize, w: usize) -> bool {
        assert!(v < self.n && w < self.n);

        self.g[v].contains(&w)
    }

    fn adj(&self, v: usize) -> Vec<usize> {
        self.g[v].clone()
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

// 有权图
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
        write!(f, "{}-{:.1}-{}", self.a, self.weight, self.b)
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
            print!("vertex {}: \t", i);
            for v in self.adj(i) {
                print!("{}\t", v);
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

fn graph_basic() {
    let n = 20;
    let m = 100;

    let mut rng = rand::thread_rng();

    // Sparse Graph
    let mut g1 = SparseGraph::new(n, false);
    for _ in 0..m {
        let a = rng.gen_range(0, n);
        let b = rng.gen_range(0, n);
        g1.add_edge(a, b);
    }

    // O(E)
    for v in 0..n {
        print!("{} : ", v);
        let adj = g1.adj(v);
        for v in adj.into_iter() {
            print!("{} ", v);
        }
        println!();
    }

    println!();

    // Dense Graph
    let mut g2 = DenseGraph::new(n, false);
    for _ in 0..m {
        let a = rng.gen_range(0, n);
        let b = rng.gen_range(0, n);
        g2.add_edge(a, b);
    }

    // O(V^2)
    for v in 0..n {
        print!("{} : ", v);
        let adj = g2.adj(v);
        for v in adj.into_iter() {
            print!("{} ", v);
        }
        println!();
    }
}

fn weighted_graph_basic() {
    let n = 20;
    let m = 100;

    let mut rng = rand::thread_rng();

    // Sparse Weighted Graph
    let mut g1 = SparseWeightedGraph::new(n, false);
    for _ in 0..m {
        let a = rng.gen_range(0, n);
        let b = rng.gen_range(0, n);
        let weighted: f32 = rng.gen();
        g1.add_edge(a, b, weighted);
    }

    // O(E)
    for v in 0..n {
        print!("{} : ", v);
        let adj = g1.adj(v);
        for v in adj.into_iter() {
            print!("{} ", v);
        }
        println!();
    }

    // Dense Weighted Graph
    let mut g2 = DenseWeightedGraph::new(n, false);
    for _ in 0..m {
        let a = rng.gen_range(0, n);
        let b = rng.gen_range(0, n);
        let weighted: f32 = rng.gen();
        g2.add_edge(a, b, weighted);
    }

    // O(V^2)
    for v in 0..n {
        print!("{} : ", v);
        let adj = g2.adj(v);
        for v in adj.into_iter() {
            print!("{} ", v);
        }
        println!();
    }
}

// 通过文件读取图
fn read_graph_from_file() {
    let filename1 = Path::new("./src/files/graph/test1.txt");
    let mut g1 = SparseGraph::new(13, false);
    readgraph::read(&mut g1, filename1).unwrap();
    println!("test g1 in Sparse Graph:");
    g1.show();

    let mut g2 = DenseGraph::new(13, false);
    readgraph::read(&mut g2, filename1).unwrap();
    println!("test g2 in Dense Graph:");
    g1.show();
}

// 计算连通分量
fn graph_connected_components() {
    let filename1 = Path::new("./src/files/graph/test1.txt");
    // test1.txt
    let mut g1 = SparseGraph::new(13, false);
    readgraph::read(&mut g1, filename1).unwrap();
    let mut component1 = components::Component::new(&g1);
    println!(
        "test1.txt, Using Sparse Graph, Component Count: {}",
        component1.count()
    );

    let mut g2 = DenseGraph::new(13, false);
    readgraph::read(&mut g2, filename1).unwrap();
    let mut component2 = components::Component::new(&g2);
    println!(
        "test1.txt, Using Dense Graph, Component Count: {}",
        component2.count()
    );

    // test2.txt
    let filename2 = Path::new("./src/files/graph/test2.txt");
    let mut g1 = SparseGraph::new(7, false);
    readgraph::read(&mut g1, filename2).unwrap();
    let mut component1 = components::Component::new(&g1);
    println!(
        "test2.txt, Using Sparse Graph, Component Count: {}",
        component1.count()
    );

    let mut g2 = DenseGraph::new(7, false);
    readgraph::read(&mut g2, filename2).unwrap();
    let mut component2 = components::Component::new(&g2);
    println!(
        "test2.txt, Using Dense Graph, Component Count: {}",
        component2.count()
    );
}

fn unweighted_graph_path_dfs() {
    let filename = Path::new("./src/files/graph/test2.txt");
    let mut g = SparseGraph::new(7, false);
    readgraph::read(&mut g, filename).unwrap();
    g.show();
    let mut path = path::Path::new(&g, 0);
    println!("Path from 0 to 6:");
    path.show_path(6);
}

fn unweighted_graph_path_bfs() {
    let filename1 = Path::new("./src/files/graph/test2.txt");
    let mut g1 = SparseGraph::new(7, false);
    readgraph::read(&mut g1, filename1).unwrap();
    g1.show();

    // 比较使用深度优先遍历和广度优先遍历获得路径的不同
    // 广度优先遍历获得的是无权图的最短路径

    let mut dfs = path::Path::new(&g1, 0);
    println!("DFS : ");
    dfs.show_path(6);

    let mut bfs = path::ShortestPath::new(&g1, 0);
    println!("BFS : ");
    println!("path length: {:?}", bfs.length(6));
    bfs.show_path(6);

    let filename2 = Path::new("./src/files/graph/test1.txt");
    let mut g2 = SparseGraph::new(13, false);
    readgraph::read(&mut g2, filename2).unwrap();
    g2.show();

    // 比较使用深度优先遍历和广度优先遍历获得路径的不同
    // 广度优先遍历获得的是无权图的最短路径

    let mut dfs = path::Path::new(&g2, 0);
    println!("DFS : ");
    dfs.show_path(3);

    let mut bfs = path::ShortestPath::new(&g2, 0);
    println!("BFS : ");
    println!("path length: {:?}", bfs.length(3));
    bfs.show_path(3);
}

pub fn run() {
    // 测试图结构
    graph_basic();

    // 过文件读取图
    read_graph_from_file();

    // 测试连通分量
    graph_connected_components();

    // 测试寻路算法
    unweighted_graph_path_dfs();

    // 测试无权图最短路径算法
    unweighted_graph_path_bfs();

    // 测试有权图
    weighted_graph_basic();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        // DenseGraph
        let mut g1 = DenseGraph::new(5, false);
        g1.add_edge(0, 1);
        g1.add_edge(0, 2);
        g1.add_edge(1, 3);

        assert_eq!(g1.has_edge(0, 1), true);
        assert_eq!(g1.n, 5);
        assert_eq!(g1.m, 3);

        // SparseGraph
        let mut g2 = SparseGraph::new(5, false);
        g2.add_edge(0, 1);
        g2.add_edge(0, 2);
        g2.add_edge(1, 3);

        assert_eq!(g2.has_edge(0, 1), true);
        assert_eq!(g2.n, 5);
        assert_eq!(g2.m, 3);
    }

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
