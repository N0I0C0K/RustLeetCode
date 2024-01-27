struct Solution;

impl Solution {
    pub fn max_number_of_alloys(
        n: i32,
        k: i32,
        budget: i32,
        composition: Vec<Vec<i32>>,
        stock: Vec<i32>,
        cost: Vec<i32>,
    ) -> i32 {
        let can = |x: i64| -> bool {
            for (idx, xc) in composition.iter().enumerate() {
                let a = xc
                    .iter()
                    .enumerate()
                    .map(|(t_idx, t)| {
                        (x * (*t as i64) - stock[t_idx] as i64).max(0) * cost[t_idx] as i64
                    })
                    .sum::<i64>();
                if a <= budget as i64 {
                    return true;
                }
            }
            false
        };
        let mut l = 0;
        let mut r = 100000000;
        while l < r - 1 {
            let mid: i64 = l + (r - l) / 2;
            if can(mid) {
                l = mid;
            } else {
                r = mid;
            }
        }
        l as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        let res = Solution::max_number_of_alloys(
            3,
            2,
            15,
            vec![vec![1, 1, 1], vec![1, 1, 10]],
            vec![0, 0, 100],
            vec![1, 2, 3],
        );
        assert_eq!(res, 5);
    }

    #[test]
    fn test2() {
        let res = Solution::max_number_of_alloys(
            4,
            4,
            17,
            vec![
                vec![10, 10, 1, 5],
                vec![9, 7, 7, 1],
                vec![6, 3, 5, 9],
                vec![2, 10, 2, 7],
            ],
            vec![9, 8, 2, 7],
            vec![9, 2, 6, 10],
        );
        assert_eq!(res, 1);
    }

    #[test]
    fn test3() {
        let res = Solution::max_number_of_alloys(
            2,
            5,
            48,
            vec![vec![6, 3], vec![9, 5], vec![1, 9], vec![1, 8], vec![3, 3]],
            vec![4, 8],
            vec![10, 1],
        );
        assert_eq!(res, 5);
    }
}
