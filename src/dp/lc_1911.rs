struct Solution;

impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        if nums.len() == 0 {
            return 0;
        }
        if nums.len() == 1 {
            return nums[0] as i64;
        }
        let mut pre = (nums[0] as i64, 0i64);
        let mut now = (0i64, 0i64);

        nums.iter().skip(1).for_each(|x| {
            let t = *x as i64;
            now.0 = pre.0.max(pre.1 + t);
            now.1 = pre.1.max(pre.0 - t);

            pre = now;
        });

        i64::max(now.0, now.1)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let nums = vec![4, 2, 5, 3];
        let res = Solution::max_alternating_sum(nums);
        assert_eq!(res, 7);
    }
}
