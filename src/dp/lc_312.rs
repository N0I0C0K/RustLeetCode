struct Solution;

impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![vec![0; n + 2]; n + 2];
        let mut pnums = vec![1; n + 2];
        for i in 0..n {
            pnums[i + 1] = nums[i];
        }

        for i in (0..n + 2).rev() {
            for j in i + 1..n + 2 {
                for k in i + 1..j {
                    let t = dp[i][k] + dp[k][j] + pnums[i] * pnums[j] * pnums[k];
                    dp[i][j] = t.max(dp[i][j]);
                }
            }
        }

        dp[0][n + 1]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let nums = vec![3, 1, 5, 8];
        let res = Solution::max_coins(nums);
        assert_eq!(res, 167);
    }
}
