use std::iter::zip;

struct Solution {}

impl Solution {
    pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        let mut tup =
            Vec::from_iter(zip(difficulty.as_slice(), profit.as_slice()).map(|(a, b)| (*a, *b)));
        tup.sort_by_key(|it| (it.0, -it.1));

        let mut prefix_max = vec![tup[0].1; tup.len()];
        for i in 1..tup.len() {
            prefix_max[i] = prefix_max[i - 1].max(tup[i].1);
        }

        let res: i32 = worker
            .iter()
            .map(|wk| match tup.binary_search_by_key(wk, |it| it.0) {
                Ok(idx) => prefix_max[idx],
                Err(idx) => {
                    if idx > 0 {
                        prefix_max[idx - 1]
                    } else {
                        0
                    }
                }
            })
            .sum();

        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        let diff = vec![2, 4, 6, 8, 10];
        let pro = vec![10, 20, 30, 40, 50];
        let worker = vec![4, 5, 6, 7];

        let res = Solution::max_profit_assignment(diff, pro, worker);
        assert_eq!(res, 100);
    }
}
