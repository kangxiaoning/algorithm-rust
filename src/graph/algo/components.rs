use crate::graph::Graph;

pub struct Component<'a, G: Graph> {
    graph: &'a G,
    visited: Vec<bool>,
    id: Vec<Option<usize>>,
    component_count: usize,
}

impl<'a, G> Component<'a, G>
where
    G: Graph,
{
    pub fn new(graph: &'a G) -> Self {
        let visited = vec![false; graph.v()];
        let id = vec![None; graph.v()];
        let component_count = 0;

        Self {
            graph,
            visited,
            id,
            component_count,
        }
    }

    // 图的深度优先遍历
    fn dfs(&mut self, v: usize) {
        self.visited[v] = true;
        self.id[v] = Some(self.component_count);
        for i in self.graph.adj(v) {
            if !self.visited[i] {
                self.dfs(i);
            }
        }
    }

    // 返回图的联通分量个数
    pub fn count(&mut self) -> usize {
        for v in 0..self.graph.v() {
            if !self.visited[v] {
                self.dfs(v);
                self.component_count += 1;
            }
        }

        self.component_count
    }

    pub fn is_connected(&self, v: usize, w: usize) -> bool {
        let max = self.graph.v();
        assert!(v < max && w < max);
        self.id[v] == self.id[w]
    }
}
