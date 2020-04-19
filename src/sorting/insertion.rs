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

// 简单测试sort的时间复杂度
// 对不同数量的元素进行排序，观察元素数量和排序时间的关系
pub fn run() {
    println!("Test for random array in 1-n .");

    let n = 100;
    let mut arr1 = util::generate_random_array(n, 1, n);
    let mut arr2 = arr1.clone();
    util::test_sort("selection sort", selection::sort, &mut arr2);
    util::test_sort("insertion sort_v1", sort_v1, &mut arr1);

    let n = 1000;
    let mut arr1 = util::generate_random_array(n, 1, n);
    let mut arr2 = arr1.clone();
    util::test_sort("selection sort", selection::sort, &mut arr2);
    util::test_sort("insertion sort_v1", sort_v1, &mut arr1);

    let n = 10000;
    let mut arr1 = util::generate_random_array(n, 1, n);
    let mut arr2 = arr1.clone();
    util::test_sort("selection sort", selection::sort, &mut arr2);
    util::test_sort("insertion sort_v1", sort_v1, &mut arr1);

    let swap_times = 10;
    println!(
        "Test for nearly ordered array, swap_times = {} .",
        swap_times
    );

    let n = 100;
    let mut arr1 = util::generate_nearly_ordered_array(n, swap_times);
    let mut arr2 = arr1.clone();
    util::test_sort("selection sort", selection::sort, &mut arr2);
    util::test_sort("insertion sort_v1", sort_v1, &mut arr1);

    let n = 1000;
    let mut arr1 = util::generate_nearly_ordered_array(n, swap_times);
    let mut arr2 = arr1.clone();
    util::test_sort("selection sort", selection::sort, &mut arr2);
    util::test_sort("insertion sort_v1", sort_v1, &mut arr1);

    let n = 10000;
    let mut arr1 = util::generate_nearly_ordered_array(n, swap_times);
    let mut arr2 = arr1.clone();
    util::test_sort("selection sort", selection::sort, &mut arr2);
    util::test_sort("insertion sort_v1", sort_v1, &mut arr1);
}
