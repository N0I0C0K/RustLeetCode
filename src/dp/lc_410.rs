struct Solution;

impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut prefix = vec![0; nums.len()];
        nums.iter().enumerate().for_each(|(idx, x)| {
            prefix[idx] = *x;
            if idx > 0 {
                prefix[idx] += prefix[idx - 1];
            }
        });
        let mut dp = vec![vec![0; (k + 1) as usize]; nums.len()];
        for i in 0..nums.len() {
            dp[i][1] = prefix[i];
            for tk in 2..=(k.min((i + 1) as i32) as usize) {
                dp[i][tk] = i32::MAX;
                for j in (tk - 2)..i {
                    dp[i][tk] = dp[i][tk].min(dp[j][tk - 1].max(prefix[i] - prefix[j]));
                }
            }
        }

        dp[nums.len() - 1][k as usize]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        let res = Solution::split_array(vec![7, 2, 5, 10, 8], 2);
        assert_eq!(res, 18);
    }
}
