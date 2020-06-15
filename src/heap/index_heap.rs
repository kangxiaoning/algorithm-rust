#[allow(dead_code)]
pub struct IndexMaxHeap<T> {
    data: Vec<Option<T>>,
    count: usize,
    capacity: usize,
    indexes: Vec<usize>,
    // 反向索引，if indexes[i] == j then reverse[j] == i
    reverse: Vec<usize>,
}

#[allow(dead_code)]
impl<T> IndexMaxHeap<T>
where
    T: Ord + Clone,
{
    pub fn with_capacity(capacity: usize) -> Self {
        let data = vec![None; capacity + 1];
        let count = 0;
        let indexes = vec![0; capacity + 1];
        let reverse = vec![0; capacity + 1];
        Self {
            data,
            count,
            capacity,
            indexes,
            reverse,
        }
    }

    pub fn size(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    // indexes保存指向data的index
    // 通过indexes获取data进行比较
    // shift_up根据对比操作indexes
    fn shift_up(&mut self, k: usize) {
        let mut k = k;
        while k > 1 && self.data[self.indexes[k / 2]] < self.data[self.indexes[k]] {
            self.indexes.swap(k / 2, k);
            self.reverse[self.indexes[k / 2]] = k / 2;
            self.reverse[self.indexes[k]] = k;
            k /= 2;
        }
    }

    pub fn insert(&mut self, index: usize, item: T) {
        assert!(self.count + 1 <= self.capacity);
        assert!(index + 1 >= 1 && index + 1 <= self.capacity);

        // 确保指定位置不存在元素
        if !self.contain(index) {
            // 内部索引从 1 开始，因此需要加 1
            let mut i = index;
            i += 1;
            self.data[i] = Some(item);
            self.count += 1;
            self.indexes[self.count] = i;
            self.reverse[i] = self.count;

            self.shift_up(self.count);
        }
    }

    fn shift_down(&mut self, k: usize) {
        let mut k = k;
        while 2 * k <= self.count {
            // 右节点的索引
            let mut j = 2 * k;
            // 如果存在右节点，并且右节点大于左节点，j 取右节点的索引
            // 比较data中的值，操作indexes中的值
            if j + 1 <= self.count && self.data[self.indexes[j + 1]] > self.data[self.indexes[j]] {
                j += 1;
            }

            // 如果 k 节点的数据大于等于任何子节点的数据，不需要处理
            if self.data[self.indexes[k]] >= self.data[self.indexes[j]] {
                break;
            }

            // 交换indexes中的索引
            self.indexes.swap(k, j);
            self.reverse[self.indexes[k]] = k;
            self.reverse[self.indexes[j]] = j;
            k = j;
        }
    }

    // 查看最大索引堆中堆顶元素
    pub fn get_max(&self) -> Option<T> {
        if self.count == 0 {
            return None;
        }

        self.data[self.indexes[1]].clone()
    }

    // pop 最大索引堆中堆顶元素
    pub fn extract_max(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }

        let ret = self.data[self.indexes[1]].clone();
        self.indexes.swap(1, self.count);
        // 删除最后一个元素，reverse中指向0，表示不存在
        self.reverse[self.indexes[self.count]] = 0;
        self.count -= 1;

        // heap还有元素，才进行reverse和heap维护
        if self.count > 0 {
            self.reverse[self.indexes[1]] = 1;
            self.shift_down(1);
        }

        ret
    }

    // pop 最大索引堆中堆顶元素的索引
    pub fn extract_max_index(&mut self) -> Option<usize> {
        if self.count == 0 {
            return None;
        }

        let ret = Some(self.indexes[1] - 1);
        self.indexes.swap(1, self.count);
        // 删除最后一个元素，reverse中指向0，表示不存在
        self.reverse[self.indexes[self.count]] = 0;
        self.count -= 1;

        // heap还有元素，才进行reverse和heap维护
        if self.count > 0 {
            self.reverse[self.indexes[1]] = 1;
            self.shift_down(1);
        }

        ret
    }

    // 查看最大索引堆中堆顶元素的索引
    pub fn get_max_index(&self) -> Option<usize> {
        if self.count == 0 {
            return None;
        }

        Some(self.indexes[1] - 1)
    }

    // 看索引i所在的位置是否存在元素
    fn contain(&self, i: usize) -> bool {
        if i + 1 >= 1 && i + 1 <= self.capacity {
            return self.reverse[i + 1] != 0;
        }
        false
    }

    // 查看最大索引堆中索引为 i 的元素
    pub fn get_item(&self, index: usize) -> Option<T> {
        // if self.count == 0 || index + 1 > self.capacity {
        //     return None;
        // }
        if self.contain(index) {
            return self.data[index + 1].clone();
        }
        None
    }

    // 将最大索引堆中索引为i的元素修改为new_item
    pub fn change(&mut self, index: usize, new_item: T) {
        assert_eq!(self.contain(index), true);
        // 确保只修改有效数据
        if self.contain(index) {
            let i = index + 1;
            self.data[i] = Some(new_item);

            // 找到indexes[j] = i, j表示data[i]在堆中的位置
            // 之后shiftUp(j), 再shiftDown(j)

            // O(n)
            // let mut j = 1;
            // while j <= self.count {
            //     if self.indexes[j] == i {
            //         self.shift_up(i);
            //         self.shift_down(i);
            //         break;
            //     }
            //     j += 1;
            // }

            // reverse优化
            // O(logn)
            self.shift_up(self.reverse[i]);
            self.shift_down(self.reverse[i]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let capacity = 0;
        let heap: IndexMaxHeap<usize> = IndexMaxHeap::with_capacity(capacity);
        assert_eq!(heap.get_max(), None);
    }

    #[test]
    fn basic() {
        let mut heap = IndexMaxHeap::with_capacity(5);
        assert_eq!(heap.is_empty(), true);
        heap.insert(0, 'a');
        heap.insert(1, 'b');
        heap.insert(2, 'c');

        assert_eq!(heap.is_empty(), false);
        assert_eq!(heap.size(), 3);
        assert_eq!(heap.get_max().unwrap(), 'c');
        assert_eq!(heap.extract_max().unwrap(), 'c');
        assert_eq!(heap.get_max().unwrap(), 'b');

        heap.insert(4, 'e');
        assert_eq!(heap.get_max().unwrap(), 'e');
        assert_eq!(heap.get_max_index().unwrap(), 4);
        assert_eq!('E' < 'e', true);
        heap.change(4, 'E');
        assert_eq!(heap.get_max().unwrap(), 'b');
        assert_eq!(heap.get_max_index().unwrap(), 1);
        assert_eq!(heap.extract_max_index().unwrap(), 1);

        assert_eq!(heap.get_item(3), None);
        assert_eq!(heap.get_item(5), None);
    }
}
