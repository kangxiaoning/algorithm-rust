// 167. Two Sum II - Input array is sorted
// Easy

// Given an array of integers that is already sorted in ascending order,
// find two numbers such that they add up to a specific target number.

// The function twoSum should return indices of the two numbers such that
// they add up to the target, where index1 must be less than index2.

// Note:

//     Your returned answers (both index1 and index2) are not zero-based.
//     You may assume that each input would have exactly one solution and
// you may not use the same element twice.

// Example:

// Input: numbers = [2,7,11,15], target = 9
// Output: [1,2]
// Explanation: The sum of 2 and 7 is 9. Therefore index1 = 1, index2 = 2.

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i = 0;
        let mut j = numbers.len() - 1;
        let ret: Vec<i32> = Vec::new();

        while i < j {
            let sum = numbers[i] + numbers[j];
            if sum == target {
                return vec![i as i32 + 1, j as i32 + 1];
            } else if sum < target {
                i += 1;
            } else {
                j -= 1;
            }
        }
        ret
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_two_sum() {
        let numbers = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(Solution::two_sum(numbers, target), vec![1, 2]);
    }
}
