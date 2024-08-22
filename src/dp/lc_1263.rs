struct Solution;

impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        let mut dp: Vec<[Option<i32>; 3]> = vec![[None; 3]; l];
        dp[0][(nums[0] % 3) as usize] = Some(nums[0]);
        for i in 1..l {
            let t = (nums[i] % 3) as usize;
            for j in 0..3 {
                dp[i][j] = (dp[i - 1][((j + 3 - t) % 3) as usize]
                    .and_then(|p| Some(p + nums[i]))
                    .or(None))
                .max(dp[i - 1][j]);
            }
            dp[i][t] = dp[i][t].max(Some(nums[i]));
        }
        dp[l - 1][0].unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let nums = vec![2, 19, 6, 16, 5, 10, 7, 4, 11, 6];
        let res = Solution::max_sum_div_three(nums);
        assert_eq!(res, 84);
    }
}
