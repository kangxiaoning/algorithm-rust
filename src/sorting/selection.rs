use crate::util;

pub fn sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();

    // 遍历数组，确定第 i 个位置应该放置的元素
    for i in 0..len {
        // 定义剩余部分最小元素的 index
        let mut min_index = i;
        // 寻找剩余范围最小元素的 index
        for j in i..len {
            if arr[j] < arr[min_index] {
                min_index = j
            }
        }
        // 当前位置元素与该范围最小元素交换
        arr.swap(i, min_index);
    }
}

// 简单测试sort的时间复杂度
// 对不同数量的元素进行排序，观察元素数量和排序时间的关系
pub fn run() {
    println!("Test for random array in 1-n .");

    let n = 100;
    let mut arr = util::generate_random_array(n, 1, n);
    util::test_sort("selection sort", sort, &mut arr);

    let n = 1000;
    let mut arr = util::generate_random_array(n, 1, n);
    util::test_sort("selection sort", sort, &mut arr);

    let n = 10000;
    let mut arr = util::generate_random_array(n, 1, n);
    util::test_sort("selection sort", sort, &mut arr);

    // 如下需要执行247秒左右
    // let n = 100000;
    // let mut arr = util::generate_random_array(n, 1, n);
    // util::test_sort("selection sort", sort, &mut arr);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer() {
        let mut res = vec![4, 1, 8, 5, 7];
        sort(&mut res);
        assert_eq!(res, vec![1, 4, 5, 7, 8]);
    }

    #[test]
    fn chars() {
        let mut res = vec!['A', 'a', 'h', 'b', 'W'];
        sort(&mut res);
        assert_eq!(res, vec!['A', 'W', 'a', 'b', 'h']);
    }

    #[test]
    fn empty() {
        let mut res = Vec::<u8>::new();
        sort(&mut res);
        assert_eq!(res, vec![]);
    }

    #[test]
    fn pre_sorted() {
        let mut res = vec!['a', 'b', 'c', 'd'];
        sort(&mut res);
        assert_eq!(res, vec!['a', 'b', 'c', 'd']);
    }
}
