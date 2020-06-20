use crate::util;

// 对arr[low:high]进行partition操作，将第1个元素放在排序后的位置
fn partition<T: Ord + Clone>(arr: &mut [T], low: usize, high: usize) -> usize {
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

fn quick_sort<T: Ord + Clone>(arr: &mut [T], low: usize, high: usize) {
    if low >= high {
        return;
    }

    let p = partition(arr, low, high);
    if p > 0 {
        quick_sort(arr, low, p - 1);
    }
    quick_sort(arr, p + 1, high);
}

pub fn sort_v1<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() > 0 {
        let high = arr.len() - 1;
        quick_sort(arr, 0, high)
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
    util::test_sort("selection sort", super::selection::sort, &mut arr1);
    util::test_sort("insertion sort_v1", super::insertion::sort_v1, &mut arr2);
    util::test_sort("insertion sort_v2", super::insertion::sort_v2, &mut arr3);
    util::test_sort("merge sort_v1", super::merge::sort_v1, &mut arr4);
    util::test_sort("heap sort_v1", super::heap::sort_v1, &mut arr5);
    util::test_sort("heap sort_v2", super::heap::sort_v2, &mut arr6);
    util::test_sort("heap sort_v3", super::heap::sort_v3, &mut arr7);
    util::test_sort("quick sort_v1", super::quick::sort_v1, &mut arr8);

    let n = 1000;
    let mut arr1 = util::generate_random_array(n, 1, n);
    let mut arr2 = arr1.clone();
    let mut arr3 = arr1.clone();
    let mut arr4 = arr1.clone();
    let mut arr5 = arr1.clone();
    let mut arr6 = arr1.clone();
    let mut arr7 = arr1.clone();
    let mut arr8 = arr1.clone();
    util::test_sort("selection sort", super::selection::sort, &mut arr1);
    util::test_sort("insertion sort_v1", super::insertion::sort_v1, &mut arr2);
    util::test_sort("insertion sort_v2", super::insertion::sort_v2, &mut arr3);
    util::test_sort("merge sort_v1", super::merge::sort_v1, &mut arr4);
    util::test_sort("heap sort_v1", super::heap::sort_v1, &mut arr5);
    util::test_sort("heap sort_v2", super::heap::sort_v2, &mut arr6);
    util::test_sort("heap sort_v3", super::heap::sort_v3, &mut arr7);
    util::test_sort("quick sort_v1", super::quick::sort_v1, &mut arr8);

    let n = 10000;
    let mut arr1 = util::generate_random_array(n, 1, n);
    let mut arr2 = arr1.clone();
    let mut arr3 = arr1.clone();
    let mut arr4 = arr1.clone();
    let mut arr5 = arr1.clone();
    let mut arr6 = arr1.clone();
    let mut arr7 = arr1.clone();
    let mut arr8 = arr1.clone();
    util::test_sort("selection sort", super::selection::sort, &mut arr1);
    util::test_sort("insertion sort_v1", super::insertion::sort_v1, &mut arr2);
    util::test_sort("insertion sort_v2", super::insertion::sort_v2, &mut arr3);
    util::test_sort("merge sort_v1", super::merge::sort_v1, &mut arr4);
    util::test_sort("heap sort_v1", super::heap::sort_v1, &mut arr5);
    util::test_sort("heap sort_v2", super::heap::sort_v2, &mut arr6);
    util::test_sort("heap sort_v3", super::heap::sort_v3, &mut arr7);
    util::test_sort("quick sort_v1", super::quick::sort_v1, &mut arr8);

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
    util::test_sort("selection sort", super::selection::sort, &mut arr1);
    util::test_sort("insertion sort_v1", super::insertion::sort_v1, &mut arr2);
    util::test_sort("insertion sort_v2", super::insertion::sort_v2, &mut arr3);
    util::test_sort("merge sort_v1", super::merge::sort_v1, &mut arr4);
    util::test_sort("heap sort_v1", super::heap::sort_v1, &mut arr5);
    util::test_sort("heap sort_v2", super::heap::sort_v2, &mut arr6);
    util::test_sort("heap sort_v3", super::heap::sort_v3, &mut arr7);
    util::test_sort("quick sort_v1", super::quick::sort_v1, &mut arr8);

    let n = 1000;
    let mut arr1 = util::generate_nearly_ordered_array(n, swap_times);
    let mut arr2 = arr1.clone();
    let mut arr3 = arr1.clone();
    let mut arr4 = arr1.clone();
    let mut arr5 = arr1.clone();
    let mut arr6 = arr1.clone();
    let mut arr7 = arr1.clone();
    let mut arr8 = arr1.clone();
    util::test_sort("selection sort", super::selection::sort, &mut arr1);
    util::test_sort("insertion sort_v1", super::insertion::sort_v1, &mut arr2);
    util::test_sort("insertion sort_v2", super::insertion::sort_v2, &mut arr3);
    util::test_sort("merge sort_v1", super::merge::sort_v1, &mut arr4);
    util::test_sort("heap sort_v1", super::heap::sort_v1, &mut arr5);
    util::test_sort("heap sort_v2", super::heap::sort_v2, &mut arr6);
    util::test_sort("heap sort_v3", super::heap::sort_v3, &mut arr7);
    util::test_sort("quick sort_v1", super::quick::sort_v1, &mut arr8);

    let n = 10000;
    let mut arr1 = util::generate_nearly_ordered_array(n, swap_times);
    let mut arr2 = arr1.clone();
    let mut arr3 = arr1.clone();
    let mut arr4 = arr1.clone();
    let mut arr5 = arr1.clone();
    let mut arr6 = arr1.clone();
    let mut arr7 = arr1.clone();
    let mut arr8 = arr1.clone();
    util::test_sort("selection sort", super::selection::sort, &mut arr1);
    util::test_sort("insertion sort_v1", super::insertion::sort_v1, &mut arr2);
    util::test_sort("insertion sort_v2", super::insertion::sort_v2, &mut arr3);
    util::test_sort("merge sort_v1", super::merge::sort_v1, &mut arr4);
    util::test_sort("heap sort_v1", super::heap::sort_v1, &mut arr5);
    util::test_sort("heap sort_v2", super::heap::sort_v2, &mut arr6);
    util::test_sort("heap sort_v3", super::heap::sort_v3, &mut arr7);
    util::test_sort("quick sort_v1", super::quick::sort_v1, &mut arr8);
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
