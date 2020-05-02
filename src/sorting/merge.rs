use crate::sorting::insertion;
use crate::sorting::selection;
use crate::util;

pub fn sort_v1<T: Ord + Copy>(arr: &mut [T]) {
    let len = arr.len();
    if len == 0 {
        return;
    }
    merge_sort(arr, 0, len - 1);
}

fn merge_sort<T: Ord + Copy>(arr: &mut [T], start: usize, end: usize) {
    // 小规模数据，使用 insertion sort
    if end - start <= 15 {
        insertion::sort_v3(arr, start, end);
        return;
    }

    let mid = start + (end - start) / 2;

    // 对左半部分进行排序
    merge_sort(arr, start, mid);
    // 对右半部分进行排序
    merge_sort(arr, mid + 1, end);
    // 对两部分进行 merge
    merge(arr, start, mid, end)
}

fn merge<T: Ord + Copy>(arr: &mut [T], start: usize, mid: usize, end: usize) {
    // 分配辅助数组
    let mut aux = Vec::with_capacity(end - start + 1);
    for i in start..=end {
        aux.insert(i, arr[i].clone());
    }

    let mut i = start;
    let mut j = mid + 1;
    for k in start..=end {
        // 如果左半部分处理完成
        if i > mid {
            arr[k] = aux[j - start];
            j += 1;
        }
        // 如果右半部分处理完成
        else if j > end {
            arr[k] = aux[i - start];
            i += 1;
        }
        // 左半部分当前元素 < 右半部分当前元素
        else if aux[i - start] < aux[j - start] {
            arr[k] = aux[i - start];
            i += 1;
        }
        // 左半部分当前元素 > 右半部分当前元素
        else {
            arr[k] = aux[j - start];
            j += 1;
        }
    }
}

pub fn run() {
    println!("Test for random array in 1-n .");

    let n = 100;
    let mut arr1 = util::generate_random_array(n, 1, n);
    let mut arr2 = arr1.clone();
    let mut arr3 = arr1.clone();
    let mut arr4 = arr1.clone();
    util::test_sort("selection sort", selection::sort, &mut arr1);
    util::test_sort("insertion sort_v1", insertion::sort_v1, &mut arr2);
    util::test_sort("insertion sort_v2", insertion::sort_v2, &mut arr3);
    util::test_sort("merge sort_v1", insertion::sort_v1, &mut arr4);

    let n = 1000;
    let mut arr1 = util::generate_random_array(n, 1, n);
    let mut arr2 = arr1.clone();
    let mut arr3 = arr1.clone();
    let mut arr4 = arr1.clone();
    util::test_sort("selection sort", selection::sort, &mut arr1);
    util::test_sort("insertion sort_v1", insertion::sort_v1, &mut arr2);
    util::test_sort("insertion sort_v2", insertion::sort_v2, &mut arr3);
    util::test_sort("merge sort_v1", insertion::sort_v1, &mut arr4);

    let n = 10000;
    let mut arr1 = util::generate_random_array(n, 1, n);
    let mut arr2 = arr1.clone();
    let mut arr3 = arr1.clone();
    let mut arr4 = arr1.clone();
    util::test_sort("selection sort", selection::sort, &mut arr1);
    util::test_sort("insertion sort_v1", insertion::sort_v1, &mut arr2);
    util::test_sort("insertion sort_v2", insertion::sort_v2, &mut arr3);
    util::test_sort("merge sort_v1", insertion::sort_v1, &mut arr4);

    let swap_times = 10;
    println!(
        "Test for nearly ordered array, swap_times = {} .",
        swap_times
    );

    let n = 100;
    let mut arr1 = util::generate_nearly_ordered_array(n, swap_times);
    let mut arr2 = arr1.clone();
    let mut arr3 = arr1.clone();
    let mut arr4 = arr1.clone();
    util::test_sort("selection sort", selection::sort, &mut arr1);
    util::test_sort("insertion sort_v1", insertion::sort_v1, &mut arr2);
    util::test_sort("insertion sort_v2", insertion::sort_v2, &mut arr3);
    util::test_sort("merge sort_v1", insertion::sort_v1, &mut arr4);

    let n = 1000;
    let mut arr1 = util::generate_nearly_ordered_array(n, swap_times);
    let mut arr2 = arr1.clone();
    let mut arr3 = arr1.clone();
    let mut arr4 = arr1.clone();
    util::test_sort("selection sort", selection::sort, &mut arr1);
    util::test_sort("insertion sort_v1", insertion::sort_v1, &mut arr2);
    util::test_sort("insertion sort_v2", insertion::sort_v2, &mut arr3);
    util::test_sort("merge sort_v1", insertion::sort_v1, &mut arr4);

    let n = 10000;
    let mut arr1 = util::generate_nearly_ordered_array(n, swap_times);
    let mut arr2 = arr1.clone();
    let mut arr3 = arr1.clone();
    let mut arr4 = arr1.clone();
    util::test_sort("selection sort", selection::sort, &mut arr1);
    util::test_sort("insertion sort_v1", insertion::sort_v1, &mut arr2);
    util::test_sort("insertion sort_v2", insertion::sort_v2, &mut arr3);
    util::test_sort("merge sort_v1", insertion::sort_v1, &mut arr4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer() {
        let mut res = vec![4, 1, 8, 5, 7];
        sort_v1(&mut res);
        assert_eq!(res, vec![1, 4, 5, 7, 8]);
    }

    #[test]
    fn chars() {
        let mut res = vec!['A', 'a', 'h', 'b', 'W'];
        sort_v1(&mut res);
        assert_eq!(res, vec!['A', 'W', 'a', 'b', 'h']);
    }

    #[test]
    fn empty() {
        let mut res = Vec::<u8>::new();
        sort_v1(&mut res);
        assert_eq!(res, vec![]);
    }

    #[test]
    fn pre_sorted() {
        let mut res = vec!['a', 'b', 'c', 'd'];
        sort_v1(&mut res);
        assert_eq!(res, vec!['a', 'b', 'c', 'd']);
    }
}
