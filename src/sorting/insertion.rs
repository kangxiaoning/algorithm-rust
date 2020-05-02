use crate::sorting::selection;
use crate::util;

pub fn sort_v1<T: Ord>(arr: &mut [T]) {
    let len = arr.len();

    for i in 1..len {
        // 在 arr[1..i] 之间寻找合适的插入位置
        for j in (1..i + 1).rev() {
            // 如果当前元素比前面的元素小刚交换位置
            if arr[j] < arr[j - 1] {
                arr.swap(j, j - 1);
            } else {
                break;
            }
        }
    }
}

// 优化点：swap 操作转为赋值操作
pub fn sort_v2<T: Ord + Copy>(arr: &mut [T]) {
    let len = arr.len();

    for i in 1..len {
        let e = arr[i];
        // 在 arr[1..i] 之间寻找合适的插入位置
        let mut left: usize = i;
        for j in (1..i + 1).rev() {
            // 如果当前元素比前面的元素小刚交换位置
            if e < arr[j - 1] {
                arr[j] = arr[j - 1];
                left -= 1;
            } else {
                break;
            }
        }
        arr[left] = e;
    }
}

pub fn sort_v3<T: Ord + Copy>(arr: &mut [T], start: usize, end: usize) {
    for i in (start + 1)..(end + 1) {
        let e = arr[i];
        // 在 arr[start+1..i] 之间寻找合适的插入位置
        let mut left: usize = i;
        for j in ((start + 1)..(i + 1)).rev() {
            // 如果当前元素比前面的元素小刚交换位置
            if e < arr[j - 1] {
                arr[j] = arr[j - 1];
                left -= 1;
            } else {
                break;
            }
        }
        arr[left] = e;
    }
}
// 简单测试sort的时间复杂度
// 对不同数量的元素进行排序，观察元素数量和排序时间的关系
pub fn run() {
    println!("Test for random array in 1-n .");

    let n = 100;
    let mut arr1 = util::generate_random_array(n, 1, n);
    let mut arr2 = arr1.clone();
    let mut arr3 = arr1.clone();
    util::test_sort("selection sort", selection::sort, &mut arr1);
    util::test_sort("insertion sort_v1", sort_v1, &mut arr2);
    util::test_sort("insertion sort_v2", sort_v2, &mut arr3);

    let n = 1000;
    let mut arr1 = util::generate_random_array(n, 1, n);
    let mut arr2 = arr1.clone();
    let mut arr3 = arr1.clone();
    util::test_sort("selection sort", selection::sort, &mut arr1);
    util::test_sort("insertion sort_v1", sort_v1, &mut arr2);
    util::test_sort("insertion sort_v2", sort_v2, &mut arr3);

    let n = 10000;
    let mut arr1 = util::generate_random_array(n, 1, n);
    let mut arr2 = arr1.clone();
    let mut arr3 = arr1.clone();
    util::test_sort("selection sort", selection::sort, &mut arr1);
    util::test_sort("insertion sort_v1", sort_v1, &mut arr2);
    util::test_sort("insertion sort_v2", sort_v2, &mut arr3);

    let swap_times = 10;
    println!(
        "Test for nearly ordered array, swap_times = {} .",
        swap_times
    );

    let n = 100;
    let mut arr1 = util::generate_nearly_ordered_array(n, swap_times);
    let mut arr2 = arr1.clone();
    let mut arr3 = arr1.clone();
    util::test_sort("selection sort", selection::sort, &mut arr1);
    util::test_sort("insertion sort_v1", sort_v1, &mut arr2);
    util::test_sort("insertion sort_v2", sort_v2, &mut arr3);

    let n = 1000;
    let mut arr1 = util::generate_nearly_ordered_array(n, swap_times);
    let mut arr2 = arr1.clone();
    let mut arr3 = arr1.clone();
    util::test_sort("selection sort", selection::sort, &mut arr1);
    util::test_sort("insertion sort_v1", sort_v1, &mut arr2);
    util::test_sort("insertion sort_v2", sort_v2, &mut arr3);

    let n = 10000;
    let mut arr1 = util::generate_nearly_ordered_array(n, swap_times);
    let mut arr2 = arr1.clone();
    let mut arr3 = arr1.clone();
    util::test_sort("selection sort", selection::sort, &mut arr1);
    util::test_sort("insertion sort_v1", sort_v1, &mut arr2);
    util::test_sort("insertion sort_v2", sort_v2, &mut arr3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer() {
        let mut res = vec![4, 1, 8, 5, 7];
        sort_v1(&mut res);
        assert_eq!(res, vec![1, 4, 5, 7, 8]);
        let mut res = vec![4, 1, 8, 5, 7];
        sort_v2(&mut res);
        assert_eq!(res, vec![1, 4, 5, 7, 8]);
        // total sorting
        let mut res = vec![4, 1, 8, 5, 7];
        sort_v3(&mut res, 0, 4);
        assert_eq!(res, vec![1, 4, 5, 7, 8]);
        // partial sorting
        let mut res = vec![4, 1, 8, 5, 7];
        sort_v3(&mut res, 1, 3);
        assert_eq!(res, vec![4, 1, 5, 8, 7]);
    }

    #[test]
    fn chars() {
        let mut res = vec!['A', 'a', 'h', 'b', 'W'];
        sort_v1(&mut res);
        assert_eq!(res, vec!['A', 'W', 'a', 'b', 'h']);
        let mut res = vec!['A', 'a', 'h', 'b', 'W'];
        sort_v2(&mut res);
        assert_eq!(res, vec!['A', 'W', 'a', 'b', 'h']);
        // total sorting
        let mut res = vec!['A', 'a', 'h', 'b', 'W'];
        sort_v3(&mut res, 0, 4);
        assert_eq!(res, vec!['A', 'W', 'a', 'b', 'h']);
        // partial sorting
        let mut res = vec!['A', 'a', 'h', 'b', 'W'];
        sort_v3(&mut res, 1, 3);
        assert_eq!(res, vec!['A', 'a', 'b', 'h', 'W']);
    }

    #[test]
    fn empty() {
        let mut res = Vec::<u8>::new();
        sort_v1(&mut res);
        assert_eq!(res, vec![]);
        let mut res = Vec::<u8>::new();
        sort_v2(&mut res);
        assert_eq!(res, vec![]);
        let mut res = Vec::<u8>::new();
        sort_v3(&mut res, 0, 0);
        assert_eq!(res, vec![]);
    }

    #[test]
    fn pre_sorted() {
        let mut res = vec!['a', 'b', 'c', 'd'];
        sort_v1(&mut res);
        assert_eq!(res, vec!['a', 'b', 'c', 'd']);
        let mut res = vec!['a', 'b', 'c', 'd'];
        sort_v2(&mut res);
        assert_eq!(res, vec!['a', 'b', 'c', 'd']);
        let mut res = vec!['a', 'b', 'c', 'd'];
        sort_v3(&mut res, 0, 3);
        assert_eq!(res, vec!['a', 'b', 'c', 'd']);
    }
}
