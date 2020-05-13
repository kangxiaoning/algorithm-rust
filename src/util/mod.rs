pub mod sst;

use rand::{self, Rng};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::str;
use std::time::Instant;

// 生成随机数组
pub fn generate_random_array(total: i32, low: i32, high: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut result = Vec::new();

    for _ in 1..total + 1 {
        result.push(rng.gen_range(low, high + 1));
    }

    result
}

// 生成部分有序的数组
// 首先生成一个含有[0...n-1]的完全有序数组, 之后随机交换swap_times对数据
// swap_times定义了数组的无序程度
// swap_times == 0 时, 数组完全有序
// swap_times 越大, 数组越趋向于无序
pub fn generate_nearly_ordered_array(total: i32, swap_times: i32) -> Vec<i32> {
    let mut result = Vec::new();
    let mut rng = rand::thread_rng();

    for i in 0..total {
        result.push(i);
    }

    for _ in 0..swap_times {
        let pos_x = rng.gen_range(0, total - 1) as usize;
        let pos_y = rng.gen_range(0, total - 1) as usize;
        result.swap(pos_x, pos_y);
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

fn first_character_index(s: &str, start: usize) -> usize {
    let mut i = start;
    while i < s.len() {
        if (s.as_bytes()[i] as char).is_alphabetic() {
            return i;
        }
        i += 1;
    }
    return s.len();
}

pub fn read_file<P>(filename: P, words: &mut Vec<String>) -> Result<(), io::Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut contents = String::new();

    for line in reader.lines() {
        contents.push_str(&line.unwrap());
        contents.push('\n');
    }

    // 简单分词
    let mut start = first_character_index(&contents, 0);
    let mut i = start + 1;
    while i <= contents.len() {
        if i == contents.len() || !(contents.as_bytes()[i] as char).is_alphabetic() {
            // 提取单词的逻辑，假设都是ASCII，转化成bytes再切片提取单词，再用from_utf8转换为string
            // &str -> &[u8] -> string -> lowercase
            let word = str::from_utf8(&contents.as_bytes()[start..i])
                .unwrap()
                .to_lowercase();
            words.push(word);
            start = first_character_index(&contents, i);
            i = start + 1;
        } else {
            i += 1;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_character_index() {
        let s = String::from("  hello world");
        assert_eq!(first_character_index(&s, 0), 2);
        assert_eq!(first_character_index(&s, 7), 8);
        let s = String::from("12345");
        assert_eq!(first_character_index(&s, 0), 5);
    }

    #[test]
    fn test_read_file() {
        let path = Path::new("./src/main.rs");
        let mut words: Vec<String> = Vec::new();
        assert_eq!(read_file(path, &mut words).unwrap(), ());
    }
}
