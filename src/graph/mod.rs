pub mod components;
pub mod path;
pub mod readgraph;

// use readgraph;

use rand::{self, Rng};
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
            print!("vertex {}: \t", i);
            for v in self.adj(i) {
                print!("{}\t", v);
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

fn graph_basic() {
    let n = 20;
    let m = 100;

    let mut rng = rand::thread_rng();

    // Sparse Graph
    let mut g1 = SparseGraph::new(n, false);
    for _ in 0..m {
        let a = rng.gen_range(0, n) as usize;
        let b = rng.gen_range(0, n) as usize;
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
        let a = rng.gen_range(0, n) as usize;
        let b = rng.gen_range(0, n) as usize;
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
}
