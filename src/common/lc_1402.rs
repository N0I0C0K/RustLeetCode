struct Solution;

impl Solution {
    pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        satisfaction.sort();

        let mut all = 0;

        let mut endfix: Vec<i32> = satisfaction
            .iter()
            .rev()
            .map(|x| {
                all += *x;
                return all;
            })
            .collect();

        endfix.reverse();

        let mut tres = satisfaction.iter().enumerate().fold(0, |acc, (idx, x)| {
            return acc + (idx + 1) as i32 * x;
        });
        let mut res = tres.max(0);
        endfix.iter().for_each(|x| {
            tres -= x;
            res = res.max(tres);
        });

        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let nums = vec![-1, -8, 0, 5, -9];
        let res = Solution::max_satisfaction(nums);
        assert_eq!(res, 14);
    }
}
