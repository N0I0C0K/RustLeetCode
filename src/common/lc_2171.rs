struct Solution;

impl Solution {
    pub fn minimum_removal(mut beans: Vec<i32>) -> i64 {
        let n = beans.len();
        beans.sort_unstable();

        let s: i64 = beans.iter().map(|x| *x as i64).sum();
        let res = (0..n)
            .map(|idx| s - (beans[idx] as i64) * ((n - idx) as i64))
            .min();
        res.unwrap().min(s)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let v = vec![4, 1, 6, 5];
        let res = Solution::minimum_removal(v);
        assert_eq!(res, 4);
    }
}
