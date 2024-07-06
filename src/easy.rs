struct Solution;

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::VecDeque;

        let l = nums.len();
        let mut stack: VecDeque<(usize, i32)> = VecDeque::new();
        let mut res: Vec<i32> = vec![-1; l];
        for (i, t) in nums.iter().enumerate() {
            while let Some((idx, x)) = stack.back() {
                if t > x {
                    let p = stack.pop_back().unwrap();
                    res[p.0] = *t;
                } else {
                    break;
                }
            }
            stack.push_back((i, *t));
        }

        for (i, t) in nums.iter().enumerate() {
            while let Some((idx, x)) = stack.back() {
                if t > x {
                    let p = stack.pop_back().unwrap();
                    if res[p.0] == -1 {
                        res[p.0] = *t;
                    }
                } else {
                    break;
                }
            }
            stack.push_back((i, *t));
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let nums = vec![1, 2, 1];
        let res = Solution::next_greater_elements(nums);
        assert_eq!(res, [2, -1, 2]);
    }
}
