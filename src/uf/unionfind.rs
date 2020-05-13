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
            uf.union_elements(a, b);
        }
        // 打印输出对这2n个操作的耗时
        println!("UF1, {} ops, {} µs", 2 * n, now.elapsed().as_micros());
    }
}

pub fn run() {
    let n = 10000;

    union_find_test_helper::test_uf1(n);
}
