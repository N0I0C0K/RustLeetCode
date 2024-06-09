struct Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        let n = nums[0] + nums[1];
        let mut res = 1;
        let l = nums.len();
        for i in (2..l).step_by(2) {
            if i < l - 1 && nums[i] + nums[i + 1] == n {
                res += 1;
            } else {
                break;
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        let nums = vec![3, 2, 1, 4, 5];
        let res = Solution::max_operations(nums);
        assert_eq!(res, 2);
    }
}
