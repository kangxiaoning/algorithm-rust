// 75. Sort Colors
// Medium

// Given an array with n objects colored red, white or blue, sort them in-place
// so that objects of the same color are adjacent, with the colors in the order
// red, white and blue.

// Here, we will use the integers 0, 1, and 2 to represent the color red, white,
// and blue respectively.

// Note: You are not suppose to use the library's sort function for this problem.

// Example:

// Input: [2,0,2,1,1,0]
// Output: [0,0,1,1,2,2]

// Follow up:

//     A rather straight forward solution is a two-pass algorithm using counting sort.
//     First, iterate the array counting number of 0's, 1's, and 2's, then overwrite array
//     with total number of 0's, then 1's and followed by 2's.
//     Could you come up with a one-pass algorithm using only constant space?

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn sort_colors(nums: &mut Vec<i32>) {
        // [0...zero] 存储 0
        let mut zero: i32 = -1;
        // [zero+1...i-1] 存储 1
        let mut i = 0;
        // n = nums.len(), [two...n) 存储 2
        let mut two = nums.len();

        while i < two {
            let e = nums[i];
            if e == 0 {
                zero += 1;
                if (zero as usize) < i {
                    nums.swap(zero as usize, i);
                }
                i += 1;
            } else if e == 1 {
                i += 1;
            } else {
                two -= 1;
                nums.swap(two, i);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sort_colors() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
    }
}
