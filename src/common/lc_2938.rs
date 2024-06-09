struct Solution;

impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        let mut n = 0_i64;
        let mut res = 0_i64;
        for (idx, c) in s.chars().enumerate() {
            if c == '0' {
                res += idx as i64 - n;
                n += 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let s = String::from("101");
        let res = Solution::minimum_steps(s);
        assert_eq!(res, 1);
    }
}
