use rand::{self, Rng};
use std::cmp::min;

#[allow(dead_code)]
pub struct MaxHeap<T> {
    data: Vec<Option<T>>,
    count: usize,
    capacity: usize,
}

// 放在impl外面方便heapify重用
fn shift_up<T>(vector: &mut Vec<T>, k: usize)
where
    T: Ord + Clone,
{
    let mut k = k;
    while k > 1 && vector[k / 2] < vector[k] {
        vector.swap(k / 2, k);
        k = k / 2;
    }
}

fn shift_down<T>(vector: &mut Vec<T>, k: usize, count: usize)
where
    T: Ord + Clone,
{
    let mut k = k;
    while 2 * k <= count {
        let mut j = 2 * k;
        if j + 1 <= count && vector[j + 1] > vector[j] {
            j += 1;
        }

        if vector[k] >= vector[j] {
            break;
        }

        vector.swap(k, j);
        k = j;
    }
}

#[allow(dead_code)]
impl<T> MaxHeap<T>
where
    T: Ord + Clone,
{
    pub fn with_capacity(capacity: usize) -> Self {
        let data = vec![None; capacity + 1];
        let count = 0;
        Self {
            data,
            count,
            capacity,
        }
    }

    pub fn with_heapify(vector: &Vec<T>) -> Self {
        let capacity = vector.len();
        let mut data = vec![None; capacity + 1];
        let count = capacity;

        for (i, v) in vector.iter().enumerate() {
            data.insert(i + 1, Some(v.clone()));
        }

        let mut i = count / 2;
        while i >= 1 {
            shift_down(&mut data, i, count);
            i -= 1;
        }

        Self {
            data,
            count,
            capacity,
        }
    }

    pub fn size(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    fn shift_up(&mut self, k: usize) {
        let mut k = k;
        while k > 1 && self.data[k / 2] < self.data[k] {
            self.data.swap(k / 2, k);
            k = k / 2;
        }
    }

    pub fn insert(&mut self, item: T) {
        assert!(self.count + 1 <= self.capacity);
        self.data.insert(self.count + 1, Some(item));
        self.count += 1;
        // self.shift_up(self.count);
        shift_up(&mut self.data, self.count);
    }

    pub fn extract_max(&mut self) -> T {
        assert!(self.count > 0);
        let ret = self.data[1].clone().unwrap();

        self.data.swap(1, self.count);
        self.count -= 1;
        // self.shift_down(1);
        shift_down(&mut self.data, 1, self.count);

        ret
    }

    pub fn get_max(&self) -> T {
        assert!(self.count > 0);
        return self.data[1].clone().unwrap();
    }
}

// 只能处理整数
#[allow(dead_code)]
pub fn print_usize_heap(heap: MaxHeap<usize>) {
    // 只能打印100个元素以内的heap
    if heap.size() >= 100 {
        println!("his print function can only work for less than 100 int");
        return;
    }

    println!("The max heap size is: {}", heap.size());
    print!("Data in the max heap: ");
    for v in heap.data.iter() {
        match v {
            Some(value) => {
                // 要求堆中的所有整数在[0, 100)的范围内
                assert!(value < &100);
                print!("{} ", value);
            }
            None => (),
        }
    }

    println!();
    println!();

    let mut n: i32 = heap.size() as i32;
    let mut max_level = 0;
    let mut number_per_level = 1;
    while n > 0 {
        max_level += 1;
        n -= number_per_level;
        number_per_level *= 2;
    }

    let max_level_number = 2usize.pow(max_level - 1);
    let mut cur_tree_max_level_number = max_level_number;
    let mut index = 1;

    // 逐行生成
    let mut level = 0;
    while level < max_level {
        // 32 is the ascii code of ' '
        const SPACE: u8 = 32;
        // line1为vector,初始为空格对应的ascii码
        // 因为只打印小于100的整数，最大为2位数字，加上中间的空格，每行需要的字符数量为max_level_number * 3 - 1
        let mut line1 = vec![SPACE; max_level_number * 3 - 1];

        // 当前行包含的整数个数
        let cur_level_number = min(heap.count - 2usize.pow(level) + 1, 2usize.pow(level));

        // 将对应位置的空格ascii编码修改为数字ascii编码
        let mut is_left = true;
        let mut index_cur_level = 0;
        while index_cur_level < cur_level_number {
            put_number_in_line(
                heap.data[index].unwrap(),
                &mut line1,
                index_cur_level,
                cur_tree_max_level_number * 3 - 1,
                is_left,
            );

            is_left = !is_left;
            index += 1;
            index_cur_level += 1;
        }

        // convert ascii code to string
        let line1 = String::from_utf8(line1).unwrap();
        // 打印该行的字符串
        println!("{}", line1);

        if level == max_level - 1 {
            break;
        }

        // 将对应位置的空格ascii编码修改为'/'和'\'对应的ascii编码
        let mut line2 = vec![SPACE; max_level_number * 3 - 1];
        let mut index_cur_level = 0;
        while index_cur_level < cur_level_number {
            put_branch_in_line(
                &mut line2,
                index_cur_level,
                cur_tree_max_level_number * 3 - 1,
            );
            index_cur_level += 1;
        }

        // 将包含ascii编码的vertor转换为string并打印
        let line2 = String::from_utf8(line2).unwrap();
        println!("{}", line2);

        cur_tree_max_level_number /= 2;
        level += 1;
    }
}

#[allow(dead_code)]
fn put_number_in_line(
    num: usize,
    line: &mut Vec<u8>,
    index_cur_level: usize,
    cur_tree_width: usize,
    is_left: bool,
) {
    let sub_tree_width = (cur_tree_width - 1) / 2;
    let offset = index_cur_level * (cur_tree_width + 1) + sub_tree_width;
    assert!(offset + 1 < line.len());
    if num >= 10 {
        // 48 is the ascii code of '0'
        line[offset + 0] = 48 + (num / 10) as u8;
        line[offset + 1] = 48 + (num % 10) as u8;
    } else {
        if is_left {
            line[offset + 0] = 48 + num as u8;
        } else {
            line[offset + 1] = 48 + num as u8;
        }
    }
}

fn put_branch_in_line(line: &mut Vec<u8>, index_cur_level: usize, cur_tree_width: usize) {
    let sub_tree_width = (cur_tree_width - 1) / 2;
    let sub_sub_tree_width = (sub_tree_width - 1) / 2;
    let offset_left = index_cur_level * (cur_tree_width + 1) + sub_sub_tree_width;
    assert!(offset_left + 1 < line.len());
    let offset_right =
        index_cur_level * (cur_tree_width + 1) + sub_tree_width + 1 + sub_sub_tree_width;
    assert!(offset_right < line.len());

    // 47 is the ascii code of '/'
    line[offset_left + 1] = 47;
    // 47 is the ascii code of '\'
    line[offset_right + 0] = 92;
}

fn make_sure_ordered() {
    let mut rng = rand::thread_rng();
    let mut max_heap = MaxHeap::new(100);

    for _ in 0..100 {
        max_heap.insert(rng.gen_range(0, 100));
    }

    let mut ordered = Vec::new();
    for i in 0..100 {
        ordered.push(max_heap.extract_max());
        print!("{} ", ordered[i]);
    }
    println!();

    for i in 1..100 {
        assert!(ordered[i - 1] >= ordered[i]);
    }
}
pub fn run() {
    let mut max_heap = MaxHeap::new(100);
    let mut rng = rand::thread_rng();

    for _ in 0..50 {
        max_heap.insert(rng.gen_range(0, 100));
    }

    print_usize_heap(max_heap);

    // 测试顺序
    make_sure_ordered();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut max_heap: MaxHeap<usize> = MaxHeap::new(100);
        max_heap.insert(1);
        max_heap.insert(2);
        max_heap.insert(3);
        assert_eq!(max_heap.size(), 3);
    }

    #[test]
    fn conversion() {
        assert_eq!(0x41 as char, 'A');

        let mut bytes = vec![0x41, 0x42, 0x43];
        bytes[0] = 0x44;
        assert_eq!(String::from_utf8(bytes).unwrap(), "DBC");

        let bytes: Vec<u8> = vec![0x31, 0x20, 0x31, 0x31];
        assert_eq!(String::from_utf8(bytes).unwrap(), "1 11");

        // u8 to ascii char
        assert_eq!(65u8 as char, 'A');

        // u8 number to ascii number
        // 1 -> "1"
        // (n + 48) as char
        assert_eq!(49u8 as char, '1');

        // u8 number to ascii number
        // 1 -> "1"
        // (n + 48) as char
        let bytes: Vec<u8> = vec![49, 32, 49, 49];
        assert_eq!(String::from_utf8(bytes).unwrap(), "1 11");
    }

    #[test]
    fn max_heap() {
        let mut rng = rand::thread_rng();
        let mut max_heap = MaxHeap::new(100);

        for _ in 0..100 {
            max_heap.insert(rng.gen_range(0, 100));
        }

        let mut ordered = Vec::new();
        for i in 0..100 {
            ordered.push(max_heap.extract_max());
            println!("{}", ordered[i]);
        }

        for i in 1..100 {
            assert!(ordered[i - 1] >= ordered[i]);
        }
    }

    #[test]
    fn heapify() {
        let mut rng = rand::thread_rng();

        // 构建随机数组
        let mut vector = Vec::new();
        for _ in 0..100 {
            vector.push(rng.gen_range(0, 100));
        }

        // heapify
        let mut max_heap = MaxHeap::with_heapify(&vector);

        let mut ordered = Vec::new();
        for i in 0..100 {
            ordered.push(max_heap.extract_max());
            print!("{} ", ordered[i]);
        }

        for i in 1..100 {
            assert!(ordered[i - 1] >= ordered[i]);
        }
    }
}
