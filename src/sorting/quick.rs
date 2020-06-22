use crate::util;
use rand::{self, Rng};

// 对arr[low:high]进行partition操作，将第1个元素放在排序后的位置
fn partition_v1<T: Ord + Clone>(arr: &mut [T], low: usize, high: usize) -> usize {
    // pick first element as pivot
    let e = arr[low].clone();

    // arr[low+1:j] < v
    let mut j = low;
    // arr[j+1:i) > v
    let mut i = low + 1;
    while i <= high {
        if arr[i].clone() < e {
            j += 1;
            arr.swap(j, i);
        }
        i += 1;
    }

    arr.swap(low, j);
    j
}

fn quick_sort_v1<T: Ord + Clone>(arr: &mut [T], low: usize, high: usize) {
    if low >= high {
        return;
    }

    let p = partition_v1(arr, low, high);
    if p > 0 {
        quick_sort_v1(arr, low, p - 1);
    }
    quick_sort_v1(arr, p + 1, high);
}

pub fn sort_v1<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() > 0 {
        let high = arr.len() - 1;
        quick_sort_v1(arr, 0, high)
    }
}

// 优化一：小规模数组使用insertion sort进行优化
fn quick_sort_v2<T: Ord + Clone>(arr: &mut [T], low: usize, high: usize) {
    if low >= high {
        return;
    }

    if high - low <= 15 {
        super::insertion::sort_v3(arr, low, high);
        return;
    }

    let p = partition_v2(arr, low, high);
    if p > 0 {
        quick_sort_v2(arr, low, p - 1);
    }
    quick_sort_v2(arr, p + 1, high);
}

// 优化二：选择随机数作为pivot
fn partition_v2<T: Ord + Clone>(arr: &mut [T], low: usize, high: usize) -> usize {
    // pick a random element as pivot
    let mut rng = rand::thread_rng();
    arr.swap(low, rng.gen_range(low, high));
    let e = arr[low].clone();

    // arr[low+1:j] < v
    let mut j = low;
    // arr[j+1:i) > v
    let mut i = low + 1;
    while i <= high {
        if arr[i].clone() < e {
            j += 1;
            arr.swap(j, i);
        }
        i += 1;
    }

    arr.swap(low, j);
    j
}

pub fn sort_v2<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() > 0 {
        let high = arr.len() - 1;
        quick_sort_v2(arr, 0, high)
    }
}

// 优化三：二路排序，均分重复元素
fn partition_v3<T: Ord + Clone>(arr: &mut [T], low: usize, high: usize) -> usize {
    // pick a random element as pivot
    let mut rng = rand::thread_rng();
    arr.swap(low, rng.gen_range(low, high));
    let e = arr[low].clone();

    // arr(j:high] >= v
    let mut j = high;
    // arr[j+1:i) <= v
    let mut i = low + 1;

    loop {
        // 不能是 arr[i] <= e，否则会导致重复元素分布不均
        while i <= high && arr[i] < e {
            i += 1;
        }

        // 不能是 arr[j] >= e，否则会导致重复元素分布不均
        while j >= low + 1 && arr[j] > e {
            j -= 1;
        }

        if i >= j {
            break;
        }

        arr.swap(i, j);
        i += 1;
        j -= 1;
    }

    arr.swap(low, j);
    j
}

fn quick_sort_v3<T: Ord + Clone>(arr: &mut [T], low: usize, high: usize) {
    if low >= high {
        return;
    }

    if high - low <= 15 {
        super::insertion::sort_v3(arr, low, high);
        return;
    }

    let p = partition_v3(arr, low, high);
    if p > 0 {
        quick_sort_v3(arr, low, p - 1);
    }
    quick_sort_v3(arr, p + 1, high);
}

pub fn sort_v3<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() > 0 {
        let high = arr.len() - 1;
        quick_sort_v3(arr, 0, high)
    }
}

// 优化四：3-ways quick sort
// 优势在于处理大量重复元素的场景
fn quick_sort_v4<T: Ord + Clone>(arr: &mut [T], low: usize, high: usize) {
    if low >= high {
        return;
    }

    if high - low <= 15 {
        super::insertion::sort_v3(arr, low, high);
        return;
    }

    // 3-ways quick sort
    let mut rng = rand::thread_rng();
    arr.swap(low, rng.gen_range(low, high));
    let e = arr[low].clone();

    // arr[low+1:lt] < e
    let mut lt = low;
    // arr[lt+1:i-1] == e
    let mut i = low + 1;
    // arr[gt:high] > e
    let mut gt = high + 1;

    while i <= high {
        if i >= gt {
            break;
        }

        // arr[low+1:lt] < e
        if arr[i] < e {
            arr.swap(i, lt + 1);
            lt += 1;
            i += 1;
        // arr[gt:high] > e
        } else if arr[i] > e {
            arr.swap(i, gt - 1);
            gt -= 1;
        // arr[lt+1:i-1] == e
        } else {
            i += 1;
        }
    }

    // make sure e in the orerdred position
    arr.swap(low, lt);
    // 此时，arr[low+1:lt-1] < e

    if lt > 0 {
        quick_sort_v4(arr, low, lt - 1);
    }
    quick_sort_v4(arr, gt, high);
}

pub fn sort_v4<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() > 0 {
        let high = arr.len() - 1;
        quick_sort_v4(arr, 0, high)
    }
}

pub fn run() {
    println!("Test for random array in 1-n .");

    let n = 100;
    let mut arr1 = util::generate_random_array(n, 1, n);
    let mut arr2 = arr1.clone();
    let mut arr3 = arr1.clone();
    let mut arr4 = arr1.clone();
    let mut arr5 = arr1.clone();
    let mut arr6 = arr1.clone();
    let mut arr7 = arr1.clone();
    let mut arr8 = arr1.clone();
    let mut arr9 = arr1.clone();
    util::test_sort("selection sort", super::selection::sort, &mut arr1);
    util::test_sort("insertion sort_v1", super::insertion::sort_v1, &mut arr2);
    util::test_sort("insertion sort_v2", super::insertion::sort_v2, &mut arr3);
    util::test_sort("merge sort_v1", super::merge::sort_v1, &mut arr4);
    util::test_sort("heap sort_v1", super::heap::sort_v1, &mut arr5);
    util::test_sort("heap sort_v2", super::heap::sort_v2, &mut arr6);
    util::test_sort("heap sort_v3", super::heap::sort_v3, &mut arr7);
    util::test_sort("quick sort_v1", super::quick::sort_v1, &mut arr8);
    util::test_sort("quick sort_v2", super::quick::sort_v2, &mut arr9);

    let n = 1000;
    let mut arr1 = util::generate_random_array(n, 1, n);
    let mut arr2 = arr1.clone();
    let mut arr3 = arr1.clone();
    let mut arr4 = arr1.clone();
    let mut arr5 = arr1.clone();
    let mut arr6 = arr1.clone();
    let mut arr7 = arr1.clone();
    let mut arr8 = arr1.clone();
    let mut arr9 = arr1.clone();
    util::test_sort("selection sort", super::selection::sort, &mut arr1);
    util::test_sort("insertion sort_v1", super::insertion::sort_v1, &mut arr2);
    util::test_sort("insertion sort_v2", super::insertion::sort_v2, &mut arr3);
    util::test_sort("merge sort_v1", super::merge::sort_v1, &mut arr4);
    util::test_sort("heap sort_v1", super::heap::sort_v1, &mut arr5);
    util::test_sort("heap sort_v2", super::heap::sort_v2, &mut arr6);
    util::test_sort("heap sort_v3", super::heap::sort_v3, &mut arr7);
    util::test_sort("quick sort_v1", super::quick::sort_v1, &mut arr8);
    util::test_sort("quick sort_v2", super::quick::sort_v2, &mut arr9);

    let n = 10000;
    let mut arr1 = util::generate_random_array(n, 1, n);
    let mut arr2 = arr1.clone();
    let mut arr3 = arr1.clone();
    let mut arr4 = arr1.clone();
    let mut arr5 = arr1.clone();
    let mut arr6 = arr1.clone();
    let mut arr7 = arr1.clone();
    let mut arr8 = arr1.clone();
    let mut arr9 = arr1.clone();
    util::test_sort("selection sort", super::selection::sort, &mut arr1);
    util::test_sort("insertion sort_v1", super::insertion::sort_v1, &mut arr2);
    util::test_sort("insertion sort_v2", super::insertion::sort_v2, &mut arr3);
    util::test_sort("merge sort_v1", super::merge::sort_v1, &mut arr4);
    util::test_sort("heap sort_v1", super::heap::sort_v1, &mut arr5);
    util::test_sort("heap sort_v2", super::heap::sort_v2, &mut arr6);
    util::test_sort("heap sort_v3", super::heap::sort_v3, &mut arr7);
    util::test_sort("quick sort_v1", super::quick::sort_v1, &mut arr8);
    util::test_sort("quick sort_v2", super::quick::sort_v2, &mut arr9);

    let n = 100000;
    let mut arr1 = util::generate_random_array(n, 1, n);
    let mut arr2 = arr1.clone();
    let mut arr3 = arr1.clone();
    let mut arr4 = arr1.clone();
    let mut arr5 = arr1.clone();
    let mut arr6 = arr1.clone();
    util::test_sort("merge sort_v1", super::merge::sort_v1, &mut arr1);
    util::test_sort("heap sort_v1", super::heap::sort_v1, &mut arr2);
    util::test_sort("heap sort_v2", super::heap::sort_v2, &mut arr3);
    util::test_sort("heap sort_v3", super::heap::sort_v3, &mut arr4);
    util::test_sort("quick sort_v1", super::quick::sort_v1, &mut arr5);
    util::test_sort("quick sort_v2", super::quick::sort_v2, &mut arr6);

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
    let mut arr5 = arr1.clone();
    let mut arr6 = arr1.clone();
    let mut arr7 = arr1.clone();
    let mut arr8 = arr1.clone();
    let mut arr9 = arr1.clone();
    util::test_sort("selection sort", super::selection::sort, &mut arr1);
    util::test_sort("insertion sort_v1", super::insertion::sort_v1, &mut arr2);
    util::test_sort("insertion sort_v2", super::insertion::sort_v2, &mut arr3);
    util::test_sort("merge sort_v1", super::merge::sort_v1, &mut arr4);
    util::test_sort("heap sort_v1", super::heap::sort_v1, &mut arr5);
    util::test_sort("heap sort_v2", super::heap::sort_v2, &mut arr6);
    util::test_sort("heap sort_v3", super::heap::sort_v3, &mut arr7);
    util::test_sort("quick sort_v1", super::quick::sort_v1, &mut arr8);
    util::test_sort("quick sort_v2", super::quick::sort_v2, &mut arr9);

    let n = 1000;
    let mut arr1 = util::generate_nearly_ordered_array(n, swap_times);
    let mut arr2 = arr1.clone();
    let mut arr3 = arr1.clone();
    let mut arr4 = arr1.clone();
    let mut arr5 = arr1.clone();
    let mut arr6 = arr1.clone();
    let mut arr7 = arr1.clone();
    let mut arr8 = arr1.clone();
    let mut arr9 = arr1.clone();
    util::test_sort("selection sort", super::selection::sort, &mut arr1);
    util::test_sort("insertion sort_v1", super::insertion::sort_v1, &mut arr2);
    util::test_sort("insertion sort_v2", super::insertion::sort_v2, &mut arr3);
    util::test_sort("merge sort_v1", super::merge::sort_v1, &mut arr4);
    util::test_sort("heap sort_v1", super::heap::sort_v1, &mut arr5);
    util::test_sort("heap sort_v2", super::heap::sort_v2, &mut arr6);
    util::test_sort("heap sort_v3", super::heap::sort_v3, &mut arr7);
    util::test_sort("quick sort_v1", super::quick::sort_v1, &mut arr8);
    util::test_sort("quick sort_v2", super::quick::sort_v2, &mut arr9);

    let n = 10000;
    let mut arr1 = util::generate_nearly_ordered_array(n, swap_times);
    let mut arr2 = arr1.clone();
    let mut arr3 = arr1.clone();
    let mut arr4 = arr1.clone();
    let mut arr5 = arr1.clone();
    let mut arr6 = arr1.clone();
    let mut arr7 = arr1.clone();
    let mut arr8 = arr1.clone();
    let mut arr9 = arr1.clone();
    util::test_sort("selection sort", super::selection::sort, &mut arr1);
    util::test_sort("insertion sort_v1", super::insertion::sort_v1, &mut arr2);
    util::test_sort("insertion sort_v2", super::insertion::sort_v2, &mut arr3);
    util::test_sort("merge sort_v1", super::merge::sort_v1, &mut arr4);
    util::test_sort("heap sort_v1", super::heap::sort_v1, &mut arr5);
    util::test_sort("heap sort_v2", super::heap::sort_v2, &mut arr6);
    util::test_sort("heap sort_v3", super::heap::sort_v3, &mut arr7);
    util::test_sort("quick sort_v1", super::quick::sort_v1, &mut arr8);
    util::test_sort("quick sort_v2", super::quick::sort_v2, &mut arr9);

    let swap_times = 100;
    println!(
        "Test for nearly ordered array, swap_times = {} .",
        swap_times
    );
    let n = 1000000;
    let mut arr1 = util::generate_nearly_ordered_array(n, swap_times);
    // let mut arr2 = arr1.clone();
    // let mut arr3 = arr1.clone();
    let mut arr4 = arr1.clone();
    // let mut arr5 = arr1.clone();
    let mut arr6 = arr1.clone();
    let mut arr7 = arr1.clone();
    let mut arr8 = arr1.clone();
    util::test_sort("merge sort_v1", super::merge::sort_v1, &mut arr1);
    // util::test_sort("heap sort_v1", super::heap::sort_v1, &mut arr2);
    // util::test_sort("heap sort_v2", super::heap::sort_v2, &mut arr3);
    util::test_sort("heap sort_v3", super::heap::sort_v3, &mut arr4);
    // util::test_sort("quick sort_v1", super::quick::sort_v1, &mut arr5);
    util::test_sort("quick sort_v2", super::quick::sort_v2, &mut arr6);
    util::test_sort("quick sort_v3", super::quick::sort_v3, &mut arr7);
    util::test_sort("quick sort_v4", super::quick::sort_v4, &mut arr8);

    println!("Test for many duplication element array, random range [0,10].");
    let n = 1000000;
    let mut arr1 = util::generate_random_array(n, 0, 10);
    // let mut arr2 = arr1.clone();
    // let mut arr3 = arr1.clone();
    let mut arr4 = arr1.clone();
    // let mut arr5 = arr1.clone();
    // let mut arr6 = arr1.clone();
    let mut arr7 = arr1.clone();
    let mut arr8 = arr1.clone();
    util::test_sort("merge sort_v1", super::merge::sort_v1, &mut arr1);
    // util::test_sort("heap sort_v1", super::heap::sort_v1, &mut arr2);
    // util::test_sort("heap sort_v2", super::heap::sort_v2, &mut arr3);
    util::test_sort("heap sort_v3", super::heap::sort_v3, &mut arr4);
    // util::test_sort("quick sort_v1", super::quick::sort_v1, &mut arr5);

    // sort_v2 跑不出来结果，报错如下，因此注释掉
    // thread 'main' has overflowed its stack
    // util::test_sort("quick sort_v2", super::quick::sort_v2, &mut arr6);
    util::test_sort("quick sort_v3", super::quick::sort_v3, &mut arr7);
    util::test_sort("quick sort_v4", super::quick::sort_v4, &mut arr8);
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

        let mut res = vec![4, 1, 8, 5, 7];
        sort_v3(&mut res);
        assert_eq!(res, vec![1, 4, 5, 7, 8]);

        let mut res = vec![4, 1, 8, 5, 7];
        sort_v4(&mut res);
        assert_eq!(res, vec![1, 4, 5, 7, 8]);
    }

    #[test]
    fn chars() {
        let mut res = vec!['A', 'a', 'h', 'b', 'W'];
        sort_v1(&mut res);
        assert_eq!(res, vec!['A', 'W', 'a', 'b', 'h']);

        let mut res = vec!['A', 'a', 'h', 'b', 'W'];
        sort_v2(&mut res);
        assert_eq!(res, vec!['A', 'W', 'a', 'b', 'h']);

        let mut res = vec!['A', 'a', 'h', 'b', 'W'];
        sort_v3(&mut res);
        assert_eq!(res, vec!['A', 'W', 'a', 'b', 'h']);

        let mut res = vec!['A', 'a', 'h', 'b', 'W'];
        sort_v4(&mut res);
        assert_eq!(res, vec!['A', 'W', 'a', 'b', 'h']);
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
        sort_v3(&mut res);
        assert_eq!(res, vec![]);

        let mut res = Vec::<u8>::new();
        sort_v4(&mut res);
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
        sort_v3(&mut res);
        assert_eq!(res, vec!['a', 'b', 'c', 'd']);

        let mut res = vec!['a', 'b', 'c', 'd'];
        sort_v4(&mut res);
        assert_eq!(res, vec!['a', 'b', 'c', 'd']);
    }
}
