struct Solution;

impl Solution {
    pub fn maximum_sum(arr: Vec<i32>) -> i32 {
        let t = i32::MIN / 2;
        let mut dp = vec![(t, t); arr.len()];
        dp[0].0 = arr[0];
        let mut res = arr[0];
        for i in 1..arr.len() {
            dp[i].0 = arr[i].max(dp[i - 1].0 + arr[i]);
            dp[i].1 = dp[i - 1].0.max(dp[i - 1].1 + arr[i]);

            res = res.max(i32::max(dp[i].0, dp[i].1));
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let a = vec![1, -2, 0, 3];
        let res = Solution::maximum_sum(a);
        assert_eq!(res, 4);
    }
}
