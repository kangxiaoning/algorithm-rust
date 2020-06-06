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
        assert_eq!(g1.v(), 5);
        assert_eq!(g1.e(), 3);

        // SparseGraph
        let mut g2 = SparseGraph::new(5, false);
        g2.add_edge(0, 1);
        g2.add_edge(0, 2);
        g2.add_edge(1, 3);

        assert_eq!(g2.has_edge(0, 1), true);
        assert_eq!(g2.v(), 5);
        assert_eq!(g2.e(), 3);
    }
}
