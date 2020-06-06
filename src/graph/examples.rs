use super::{
    components, path, readgraph, DenseGraph, DenseWeightedGraph, Graph, SparseGraph,
    SparseWeightedGraph, WeightedGraph,
};
use rand::{self, Rng};
use std::path::Path;

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
    g2.show();
}

// 通过文件读取有权图
fn read_weighted_graph_from_file() {
    let filename1 = Path::new("./src/files/graph/test3.txt");
    let mut g1: SparseWeightedGraph<f32> = SparseWeightedGraph::new(8, false);
    readgraph::read_weighted_graph(&mut g1, filename1).unwrap();
    println!("test g1 in Sparse Weighted Graph:");
    g1.show();

    let mut g2: DenseWeightedGraph<f32> = DenseWeightedGraph::new(8, false);
    readgraph::read_weighted_graph(&mut g2, filename1).unwrap();
    println!("test g2 in Dense Weighted Graph:");
    g2.show();
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

    // 通过文件读取图
    read_graph_from_file();
    read_weighted_graph_from_file();

    // 测试连通分量
    graph_connected_components();

    // 测试寻路算法
    unweighted_graph_path_dfs();

    // 测试无权图最短路径算法
    unweighted_graph_path_bfs();

    // 测试有权图
    weighted_graph_basic();
}
