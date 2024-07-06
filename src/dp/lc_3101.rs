struct Solution;

impl Solution {
    pub fn count_alternating_subarrays(nums: Vec<i32>) -> i64 {
        let l = nums.len();
        let mut pre = 1;
        let mut res = 1_i64;
        for i in 1..l {
            if nums[i] != nums[i - 1] {
                pre = pre + 1;
                res += pre
            } else {
                pre = 1;
                res += pre;
            }
        }
        res
    }
}
