use super::Graph;

use std::collections::VecDeque;

pub struct Path<'a, G: Graph> {
    graph: &'a G,
    visited: Vec<bool>,
    start: usize,
    from: Vec<Option<usize>>,
    initialized: bool,
}

impl<'a, G> Path<'a, G>
where
    G: Graph,
{
    pub fn new(graph: &'a G, start: usize) -> Self {
        let visited = vec![false; graph.v()];
        let from = vec![None; graph.v()];
        let initialized = false;

        Self {
            graph,
            visited,
            start,
            from,
            initialized,
        }
    }

    // 图的深度优先遍历
    fn dfs(&mut self, start: usize) {
        self.visited[start] = true;
        for i in self.graph.adj(start) {
            if !self.visited[i] {
                self.from[i] = Some(start);
                self.dfs(i);
            }
        }
    }

    // 寻路算法
    fn initialize(&mut self) {
        self.dfs(self.start);
        self.initialized = true;
    }

    // 查询从s点到w点是否有路径
    pub fn has_path(&mut self, w: usize) -> bool {
        assert!(w < self.graph.v());
        if !self.initialized {
            self.initialize();
        }

        self.visited[w]
    }

    // 查询从s点到w点的路径, 存放在vec中
    pub fn path(&mut self, w: usize, vec: &mut Vec<usize>) {
        assert_eq!(self.has_path(w), true);

        let mut stack = Vec::new();

        // 通过from数组逆向查找到从s到w的路径, 存放到栈中
        let mut p = Some(w);
        while let Some(v) = p {
            stack.push(v);
            p = self.from[v];
        }

        // 从栈中依次取出元素, 获得顺序的从s到w的路径
        while let Some(v) = stack.pop() {
            vec.push(v);
        }
    }

    // 打印出从s点到w点的路径
    pub fn show_path(&mut self, w: usize) {
        assert_eq!(self.has_path(w), true);

        let mut vec: Vec<usize> = Vec::new();
        self.path(w, &mut vec);

        for (i, v) in vec.iter().enumerate() {
            print!("{}", v);
            if i == vec.len() - 1 {
                println!();
            } else {
                print!(" -> ");
            }
        }
    }
}

// 寻找无权图的最短路径
pub struct ShortestPath<'a, G: Graph> {
    graph: &'a G,
    visited: Vec<bool>,
    start: usize,
    from: Vec<Option<usize>>,
    initialized: bool,
    order: Vec<Option<usize>>,
}

impl<'a, G> ShortestPath<'a, G>
where
    G: Graph,
{
    pub fn new(graph: &'a G, start: usize) -> Self {
        let visited = vec![false; graph.v()];
        let from = vec![None; graph.v()];
        let order = vec![None; graph.v()];
        let initialized = false;

        Self {
            graph,
            visited,
            start,
            from,
            initialized,
            order,
        }
    }

    // 图的广度优先遍历
    fn bfs(&mut self, start: usize) {
        let mut queue = VecDeque::with_capacity(self.graph.v());
        queue.push_back(start);
        self.visited[start] = true;
        self.order[start] = Some(0);

        while let Some(v) = queue.pop_front() {
            for i in self.graph.adj(v) {
                if !self.visited[i] {
                    queue.push_back(i);
                    self.visited[i] = true;
                    self.from[i] = Some(v);
                    self.order[i] = Some(self.order[v].unwrap() + 1);
                }
            }
        }
    }

    // 寻路算法
    fn initialize(&mut self) {
        self.bfs(self.start);
        self.initialized = true;
    }

    // 查询从s点到w点是否有路径
    pub fn has_path(&mut self, w: usize) -> bool {
        assert!(w < self.graph.v());
        if !self.initialized {
            self.initialize();
        }

        self.visited[w]
    }

    // 查询从s点到w点的路径, 存放在vec中
    pub fn path(&mut self, w: usize, vec: &mut Vec<usize>) {
        assert_eq!(self.has_path(w), true);

        let mut stack = Vec::new();

        // 通过from数组逆向查找到从s到w的路径, 存放到栈中
        let mut p = Some(w);
        while let Some(v) = p {
            stack.push(v);
            p = self.from[v];
        }

        // 从栈中依次取出元素, 获得顺序的从s到w的路径
        while let Some(v) = stack.pop() {
            vec.push(v);
        }
    }

    // 打印出从s点到w点的路径
    pub fn show_path(&mut self, w: usize) {
        assert_eq!(self.has_path(w), true);

        let mut vec: Vec<usize> = Vec::new();
        self.path(w, &mut vec);

        for (i, v) in vec.iter().enumerate() {
            print!("{}", v);
            if i == vec.len() - 1 {
                println!();
            } else {
                print!(" -> ");
            }
        }
    }

    // 查看从s点到w点的最短路径长度
    pub fn length(&mut self, w: usize) -> Option<usize> {
        assert!(w < self.graph.v());
        if !self.initialized {
            self.initialize();
        }
        self.order[w]
    }
}
