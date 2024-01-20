struct Solution;

impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        let k = k as usize;
        let n = n as usize;
        let target = target as usize;
        let mut dp = vec![vec![0; target + 1]; n + 1];
        let mood = 1_000_000_007;
        dp[0][0] = 1;

        for i in 1..=n {
            for j in i..=((i * k).min(target)) {
                let temp: i32 = (1..=k)
                    .map(|offset| {
                        if j >= offset && j - offset >= i - 1 {
                            return dp[i - 1][j - offset];
                        }
                        0
                    })
                    .fold(0, |acc, x| (acc + x) % mood);
                dp[i][j] = temp;
            }
        }

        dp[n][target]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let res = Solution::num_rolls_to_target(1, 6, 3);
        assert_eq!(res, 1);
    }

    #[test]
    fn test1() {
        let res = Solution::num_rolls_to_target(2, 6, 7);
        assert_eq!(res, 6);
    }
}
