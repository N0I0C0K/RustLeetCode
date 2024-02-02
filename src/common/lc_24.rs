struct Solution;

// 动态区间中位数

impl Solution {
    pub fn nums_game(nums: Vec<i32>) -> Vec<i32> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut lower: BinaryHeap<i32> = BinaryHeap::new();
        let mut upper: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        let (mut lowsum, mut uppersum) = (0_i64, 0_i64);
        let mmod = 1000000007;
        nums.iter()
            .enumerate()
            .map(|(idx, x)| {
                let p = *x - idx as i32;
                if lower.is_empty() || lower.peek().is_some_and(|ltop| *ltop >= p) {
                    lower.push(p);
                    lowsum += p as i64;
                    if lower.len() > upper.len() + 1 {
                        let tmp = lower.pop().unwrap();
                        lowsum -= tmp as i64;
                        upper.push(Reverse(tmp));
                        uppersum += tmp as i64;
                    }
                } else {
                    upper.push(Reverse(p));
                    lowsum += p as i64;
                    if upper.len() > lower.len() {
                        let tmp = upper.pop().unwrap();
                        uppersum -= tmp.0 as i64;
                        lower.push(tmp.0);
                        lowsum += tmp.0 as i64;
                    }
                }
                if (idx + 1) % 2 == 0 {
                    return ((uppersum - lowsum) % mmod) as i32;
                } else {
                    return ((uppersum - lowsum + *lower.peek().unwrap() as i64) % mmod) as i32;
                }
            })
            .collect::<Vec<i32>>()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        let res = Solution::nums_game(vec![3, 4, 5, 1, 6, 7]);
        assert_eq!(res, vec![0, 0, 0, 5, 6, 7]);
    }
}
