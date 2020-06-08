// 88. Merge Sorted Array
// Easy

// Given two sorted integer arrays nums1 and nums2, merge nums2 into nums1 as one sorted array.

// Note:

//     The number of elements initialized in nums1 and nums2 are m and n respectively.
//     You may assume that nums1 has enough space (size that is greater or equal to m + n)
//     to hold additional elements from nums2.

// Example:

// Input:
// nums1 = [1,2,3,0,0,0], m = 3
// nums2 = [2,5,6],       n = 3

// Output: [1,2,2,3,5,6]

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut aux = Vec::with_capacity(nums1.len());

        while i < m as usize && j < n as usize {
            let x = nums1[i];
            let y = nums2[j];
            if x <= y {
                aux.push(x);
                i += 1;
            } else {
                aux.push(y);
                j += 1;
            }
        }

        // 处理 nums1 中剩余元素
        while i < m as usize {
            aux.push(nums1[i]);
            i += 1;
        }

        // 处理nums2 中还有元素
        while j < n as usize {
            aux.push(nums2[j]);
            j += 1;
        }

        // 合并到 nums1
        nums1.clear();
        for e in aux.into_iter() {
            nums1.push(e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_88() {
        let mut nums1 = vec![1, 2, 3, 4, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        let m = 4;
        let n = 3;
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 2, 2, 3, 4, 5, 6]);
    }
}
