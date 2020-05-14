pub mod uf1 {
    pub struct UnionFind {
        id: Vec<usize>,
        count: usize,
    }

    impl UnionFind {
        pub fn new(n: usize) -> Self {
            // 初始化, 每一个id[i]指向自己, 表示每一个元素自己自成一个集合
            let mut id = vec![0; n];
            for i in 0..n {
                id[i] = i;
            }

            Self { id, count: n }
        }

        // 查找元素p所对应的集合编号
        pub fn find(&self, p: usize) -> usize {
            assert!(p < self.count);
            self.id[p]
        }

        // 查看元素p和元素q是否属于一个集合
        // O(1) 复杂度
        pub fn is_connected(&self, p: usize, q: usize) -> bool {
            self.find(p) == self.find(q)
        }

        // 合并元素p和q所属的集合
        // O(n) 复杂度
        pub fn union_elements(&mut self, p: usize, q: usize) {
            let p_id = self.find(p);
            let q_id = self.find(q);

            if p_id == q_id {
                return;
            }

            // 合并过程需要遍历所有元素，将两个元素的所属集合编号合并
            for i in 0..self.count {
                if self.id[i] == p_id {
                    self.id[i] = q_id;
                }
            }
        }
    }
}

// 第二版Union-Find
pub mod uf2 {

    // 我们的第二版Union-Find, 使用一个数组构建一棵指向父节点的树
    // parent[i]表示第i个元素所指向的父节点
    pub struct UnionFind {
        parent: Vec<usize>,
        count: usize,
    }

    impl UnionFind {
        pub fn new(n: usize) -> Self {
            // 初始化, 每一个parent[i]指向自己, 表示每一个元素自己自成一个集合
            let mut parent = vec![0; n];
            for i in 0..n {
                parent[i] = i;
            }
            Self { parent, count: n }
        }

        // 查找过程, 查找元素p所对应的集合编号
        // O(h)复杂度, h为树的高度
        pub fn find(&self, p: usize) -> usize {
            assert!(p < self.count);
            // 不断去查询自己的父亲节点, 直到到达根节点
            // 根节点的特点: parent[p] == p
            let mut p = p;
            while p != self.parent[p] {
                p = self.parent[p];
                // println!("p = {}", p);
            }
            p
        }

        // 查看元素p和元素q是否所属一个集合
        // O(h)复杂度, h为树的高度
        pub fn is_connected(&self, p: usize, q: usize) -> bool {
            self.find(p) == self.find(q)
        }

        // 合并元素p和元素q所属的集合
        // O(h)复杂度, h为树的高度
        pub fn union_elements(&mut self, p: usize, q: usize) {
            let p_root = self.find(p);
            let q_root = self.find(q);

            if p_root == q_root {
                return;
            }

            self.parent[p_root] = q_root;
        }
    }
}

pub mod union_find_test_helper {
    use rand::{self, Rng};
    use std::time::Instant;

    pub fn test_uf1(n: usize) {
        let mut uf = super::uf1::UnionFind::new(n);
        let mut rng = rand::thread_rng();

        // 测试运行时间
        let now = Instant::now();

        // 进行n次操作，每次随机选择两个元素进行合并操作
        for _ in 0..n {
            let a = rng.gen_range(0, n);
            let b = rng.gen_range(0, n);
            uf.union_elements(a, b);
        }

        // 进行n次操作，每次随机选择两个元素，查询是否属于同一个集合
        for _ in 0..n {
            let a = rng.gen_range(0, n);
            let b = rng.gen_range(0, n);
            uf.is_connected(a, b);
        }
        // 打印输出对这2n个操作的耗时
        println!("UF1, {} ops, {} µs", 2 * n, now.elapsed().as_micros());
    }

    pub fn test_uf2(n: usize) {
        let mut uf = super::uf2::UnionFind::new(n);
        let mut rng = rand::thread_rng();

        // 测试运行时间
        let now = Instant::now();

        // 进行n次操作，每次随机选择两个元素进行合并操作
        for _ in 0..n {
            let a = rng.gen_range(0, n);
            let b = rng.gen_range(0, n);
            uf.union_elements(a, b);
        }

        // 进行n次操作，每次随机选择两个元素，查询是否属于同一个集合
        for _ in 0..n {
            let a = rng.gen_range(0, n);
            let b = rng.gen_range(0, n);
            uf.is_connected(a, b);
        }
        // 打印输出对这2n个操作的耗时
        println!("UF2, {} ops, {} µs", 2 * n, now.elapsed().as_micros());
    }
}

pub fn run() {
    let n = 10000;

    union_find_test_helper::test_uf1(n);
    union_find_test_helper::test_uf2(n);
}
