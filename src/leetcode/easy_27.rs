// 27. Remove Element
// Easy

// Given an array nums and a value val, remove all instances of that value in-place
// and return the new length.

// Do not allocate extra space for another array, you must do this by modifying the
// input array in-place with O(1) extra memory.

// The order of elements can be changed. It doesn't matter what you leave beyond
// the new length.

// Example 1:

// Given nums = [3,2,2,3], val = 3,

// Your function should return length = 2, with the first two elements of nums being 2.

// It doesn't matter what you leave beyond the returned length.

// Example 2:

// Given nums = [0,1,2,2,3,0,4,2], val = 2,

// Your function should return length = 5, with the first five elements of nums
// containing 0, 1, 3, 0, and 4.

// Note that the order of those five elements can be arbitrary.

// It doesn't matter what values are set beyond the returned length.

// Clarification:

// Confused why the returned value is an integer but your answer is an array?

// Note that the input array is passed in by reference, which means modification
// to the input array will be known to the caller as well.

// Internally you can think of this:

// // nums is passed in by reference. (i.e., without making a copy)
// int len = removeElement(nums, val);

// // any modification to nums in your function would be known by the caller.
// // using the length returned by your function, it prints the first len elements.
// for (int i = 0; i < len; i++) {
//     print(nums[i]);
// }

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        // [0...k) 存储要保留的元素
        // [k...i) 存储要删除的元素
        let mut k = 0;

        for i in 0..nums.len() {
            // 遍历每个元素，如果不等于 val，则 swap 到 [0...k] 部分
            if nums[i] != val {
                // 前面有待删除元素，则 swap
                if i != k {
                    nums.swap(i, k);
                    k += 1;
                // 前面没有待删除元素，则 k + 1，确保 [0...k] 部分都是要保留元素
                } else {
                    k += 1;
                }
            }
        }

        k as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_27() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val = 2;
        let len = Solution::remove_element(&mut nums, val);
        assert_eq!(len, 5);
        assert_eq!(&nums[..5], &[0, 1, 3, 0, 4]);
    }
}
