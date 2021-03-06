use crate::sorting::insertion;
use crate::sorting::selection;
use crate::util;

pub fn sort_v1<T: Ord>(arr: &mut [T]) {
    let len = arr.len();

    if len == 0 {
        return;
    }

    for i in 0..len - 1 {
        for j in 0..len - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1)
            }
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
    util::test_sort("bubble sort_v1", sort_v1, &mut arr4);

    let n = 1000;
    let mut arr1 = util::generate_random_array(n, 1, n);
    let mut arr2 = arr1.clone();
    let mut arr3 = arr1.clone();
    let mut arr4 = arr1.clone();
    util::test_sort("selection sort", selection::sort, &mut arr1);
    util::test_sort("insertion sort_v1", insertion::sort_v1, &mut arr2);
    util::test_sort("insertion sort_v2", insertion::sort_v2, &mut arr3);
    util::test_sort("bubble sort_v1", sort_v1, &mut arr4);

    let n = 10000;
    let mut arr1 = util::generate_random_array(n, 1, n);
    let mut arr2 = arr1.clone();
    let mut arr3 = arr1.clone();
    let mut arr4 = arr1.clone();
    util::test_sort("selection sort", selection::sort, &mut arr1);
    util::test_sort("insertion sort_v1", insertion::sort_v1, &mut arr2);
    util::test_sort("insertion sort_v2", insertion::sort_v2, &mut arr3);
    util::test_sort("bubble sort_v1", sort_v1, &mut arr4);

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
    util::test_sort("bubble sort_v1", sort_v1, &mut arr4);

    let n = 1000;
    let mut arr1 = util::generate_nearly_ordered_array(n, swap_times);
    let mut arr2 = arr1.clone();
    let mut arr3 = arr1.clone();
    let mut arr4 = arr1.clone();
    util::test_sort("selection sort", selection::sort, &mut arr1);
    util::test_sort("insertion sort_v1", insertion::sort_v1, &mut arr2);
    util::test_sort("insertion sort_v2", insertion::sort_v2, &mut arr3);
    util::test_sort("bubble sort_v1", sort_v1, &mut arr4);

    let n = 10000;
    let mut arr1 = util::generate_nearly_ordered_array(n, swap_times);
    let mut arr2 = arr1.clone();
    let mut arr3 = arr1.clone();
    let mut arr4 = arr1.clone();
    util::test_sort("selection sort", selection::sort, &mut arr1);
    util::test_sort("insertion sort_v1", insertion::sort_v1, &mut arr2);
    util::test_sort("insertion sort_v2", insertion::sort_v2, &mut arr3);
    util::test_sort("bubble sort_v1", sort_v1, &mut arr4);
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
