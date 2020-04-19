use rand;
use rand::Rng;
use std::time::Instant;

// 生成随机数组
pub fn generate_random_array(count: i32, low: i32, high: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut result = Vec::new();

    for _ in 1..count + 1 {
        result.push(rng.gen_range(low, high + 1));
    }

    result
}

// 判断arr是否有序
pub fn is_sorted<T: Ord>(arr: &[T]) -> bool {
    let len = arr.len();
    for i in 0..len - 1 {
        if arr[i] > arr[i + 1] {
            return false;
        }
    }
    true
}

// 测试排序算法的正确性和运行时间
pub fn test_sort<T, F>(name: &str, f: F, arr: &mut [T])
where
    T: Ord,
    F: Fn(&mut [T]),
{
    // 测试运行时间
    let now = Instant::now();
    f(arr);
    println!(
        "| {:<20} | n = {:<8} | {:>12} µs |",
        name,
        arr.len(),
        now.elapsed().as_micros()
    );
    // 测试正确性
    assert_eq!(is_sorted(arr), true);
}
