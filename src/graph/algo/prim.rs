use crate::graph::{Edge, WeightedGraph};
use std::cmp::{Ord, Ordering, Reverse};
use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::fmt::{self, Display};
use std::ops::AddAssign;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Float(pub f64);

impl Eq for Float {}

impl PartialOrd for Float {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for Float {
    fn cmp(&self, other: &Float) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl AddAssign for Float {
    fn add_assign(&mut self, other: Self) {
        // *self = Self {
        //     x: self.x + other.x,
        //     y: self.y + other.y,
        // };
        self.0.add_assign(other.0);
    }
}

impl FromStr for Float {
    type Err = std::num::ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f: f64 = f64::from_str(&s)?;

        Ok(Float(f))
    }
}

pub struct LazyPrimMST<'a, W, G>
where
    G: WeightedGraph<W>,
{
    graph: &'a G,
    // make BinaryHeap to a min-heap
    pq: BinaryHeap<Reverse<Edge<W>>>,
    marked: Vec<bool>,
    mst: VecDeque<Edge<W>>,
    mst_weight: Option<W>,
    initialized: bool,
}

impl<'a, W, G> LazyPrimMST<'a, W, G>
where
    G: WeightedGraph<W>,
    W: Display + Ord + Clone + AddAssign,
{
    pub fn new(graph: &'a G) -> Self {
        let marked = vec![false; graph.v()];
        let mst = VecDeque::new();
        let pq = BinaryHeap::with_capacity(graph.v());

        Self {
            graph,
            pq,
            marked,
            mst,
            mst_weight: None,
            initialized: false,
        }
    }

    fn visit(&mut self, v: usize) {
        // assert!(!self.marked[v]);
        assert_eq!(self.marked[v], false);
        self.marked[v] = true;

        // 将和节点v相连接的所有未访问的边放入最小堆中
        for &e in self.graph.adj(v).iter() {
            if !self.marked[e.other(v)] {
                self.pq.push(Reverse(e.clone()));
            }
        }
    }

    fn lazy_prim(&mut self) {
        self.visit(0);

        while !self.pq.is_empty() {
            // 使用最小堆找出已经访问的边中权值最小的边
            let Reverse(e) = self.pq.pop().unwrap();
            // 如果这条边的两端都已经访问过了, 则扔掉这条边
            if self.marked[e.v()] == self.marked[e.w()] {
                continue;
            }
            // 否则, 这条边则应该存在在最小生成树中
            self.mst.push_back(e.clone());

            // 访问和这条边连接的还没有被访问过的节点
            if !self.marked[e.v()] {
                self.visit(e.v());
            } else {
                self.visit(e.w());
            }
        }

        // 计算最小生成树的权值
        let mut mst_weight = self.mst[0].weight();
        for i in 1..self.mst.len() {
            mst_weight += self.mst[i].weight();
        }
        self.mst_weight = Some(mst_weight);
    }

    fn initialize(&mut self) {
        self.lazy_prim();
        self.initialized = true;
    }

    pub fn result(&mut self) -> Option<W> {
        if !self.initialized {
            self.initialize();
        }

        // match &self.mst_weight {
        //     Some(v) => Some(v.clone()),
        //     None => None,
        // }
        self.mst_weight.clone()
    }

    pub fn mst_edges(&mut self) -> Vec<Edge<W>> {
        if !self.initialized {
            self.initialize();
        }

        let mut mst = Vec::new();

        for edge in self.mst.iter() {
            mst.push(edge.clone());
        }

        mst
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{readgraph, SparseWeightedGraph};
    use std::path::Path;

    #[test]
    fn mst_weight() {
        let filename = Path::new("./src/files/graph/test3.txt");
        let mut g: SparseWeightedGraph<Float> = SparseWeightedGraph::new(8, false);
        readgraph::read_weighted_graph(&mut g, filename).unwrap();

        let mut mst = LazyPrimMST::new(&g);

        assert_eq!(mst.result(), Some(Float(1.81)));
    }

    #[test]
    fn float() {
        let f1 = Float(0.26);
        let f2 = Float(0.35);
        let f3 = Float(0.58);
        let f4 = Float(0.19);
        let f5 = Float(0.78);

        let mut min_heap = BinaryHeap::new();
        min_heap.push(Reverse(f1));
        min_heap.push(Reverse(f2));
        min_heap.push(Reverse(f3));
        min_heap.push(Reverse(f4));
        min_heap.push(Reverse(f5));

        assert_eq!(min_heap.pop(), Some(Reverse(Float(0.19))));
    }
}
