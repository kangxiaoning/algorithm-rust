// 26. Remove Duplicates from Sorted Array
// Easy

// Given a sorted array nums, remove the duplicates in-place such that
// each element appear only once and return the new length.

// Do not allocate extra space for another array, you must do this by
// modifying the input array in-place with O(1) extra memory.

// Example 1:

// Given nums = [1,1,2],

// Your function should return length = 2, with the first two elements of
// nums being 1 and 2 respectively.

// It doesn't matter what you leave beyond the returned length.

// Example 2:

// Given nums = [0,0,1,1,1,2,2,3,3,4],

// Your function should return length = 5, with the first five elements of
// nums being modified to 0, 1, 2, 3, and 4 respectively.

// It doesn't matter what values are set beyond the returned length.

// Clarification:

// Confused why the returned value is an integer but your answer is an array?

// Note that the input array is passed in by reference, which means modification
// to the input array will be known to the caller as well.

// Internally you can think of this:

// // nums is passed in by reference. (i.e., without making a copy)
// int len = removeDuplicates(nums);

// // any modification to nums in your function would be known by the caller.
// // using the length returned by your function, it prints the first len elements.
// for (int i = 0; i < len; i++) {
//     print(nums[i]);
// }

// fn main() {
//     let mut nums = vec![1, 1, 2];
//     let len = Solution::remove_duplicates(&mut nums);
//     println!("Output: {}, {:?}, {:?}", len, &nums[..len as usize], nums);

//     let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
//     let len = Solution::remove_duplicates(&mut nums);
//     println!("Output: {}, {:?}, {:?}", len, &nums[..len as usize], nums);
// }

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        // 测试用例有空数组
        if nums.len() < 1 {
            return 0;
        }
        // [0...k] 存储不重复的元素
        let mut k = 0;

        // 保持 [0...k] 有序，nums[i] != nums[k] 时将 nums[i] 加入到 [0...k]，k + 1
        // 否则对比下一个元素
        for i in 1..nums.len() {
            if nums[i] != nums[k] {
                k += 1;
                nums.swap(k, i);
            }
        }

        k as i32 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_26() {
        let mut nums = vec![1, 1, 2];
        let len = Solution::remove_duplicates(&mut nums);
        assert_eq!(len, 2);
        assert_eq!(&nums[..len as usize], &[1, 2]);

        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let len = Solution::remove_duplicates(&mut nums);
        assert_eq!(len, 5);
        assert_eq!(&nums[..len as usize], &[0, 1, 2, 3, 4]);
    }
}
