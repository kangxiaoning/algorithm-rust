use rand::{self, Rng};
// 稠密图 - 邻接矩阵
pub struct DenseGraph {
    n: usize,
    m: usize,
    directed: bool,
    g: Vec<Vec<bool>>,
}

impl DenseGraph {
    pub fn new(n: usize, directed: bool) -> Self {
        // g初始化为n*n的布尔矩阵, g[i][j]为false, 表示没有任和边
        let g = vec![vec![false; n]; n];
        let m = 0;

        Self { n, m, directed, g }
    }

    // 返回节点个数
    pub fn v(&self) -> usize {
        self.n
    }

    // 返回边的个数
    pub fn e(&self) -> usize {
        self.m
    }

    pub fn add_edge(&mut self, v: usize, w: usize) {
        assert!(v < self.n && w < self.n);

        self.g[v][w] = true;
        if !self.directed {
            self.g[w][v] = true;
        }

        self.m += 1;
    }

    pub fn has_edge(&self, v: usize, w: usize) -> bool {
        assert!(v < self.n && w < self.n);
        self.g[v][w]
    }

    pub fn adj(&self, v: usize) -> Vec<usize> {
        let mut ret = Vec::new();

        for (idx, &is_true) in self.g[v].iter().enumerate() {
            if is_true {
                ret.push(idx);
            }
        }
        ret
    }
}

// 稀梳图 - 邻接表
pub struct SparseGraph {
    n: usize,
    m: usize,
    directed: bool,
    g: Vec<Vec<usize>>,
}

impl SparseGraph {
    pub fn new(n: usize, directed: bool) -> Self {
        let g = vec![vec![]; n];
        let m = 0;

        Self { n, m, directed, g }
    }

    // 返回节点个数
    pub fn v(&self) -> usize {
        self.n
    }

    // 返回边的个数
    pub fn e(&self) -> usize {
        self.m
    }

    pub fn add_edge(&mut self, v: usize, w: usize) {
        assert!(v < self.n && w < self.n);

        self.g[v].push(w);
        if v != w && !self.directed {
            self.g[w].push(v);
        }

        self.m += 1;
    }

    pub fn has_edge(&self, v: usize, w: usize) -> bool {
        assert!(v < self.n && w < self.n);

        self.g[v].contains(&w)
    }

    pub fn adj(&self, v: usize) -> Vec<usize> {
        self.g[v].clone()
    }
}

pub fn run() {
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