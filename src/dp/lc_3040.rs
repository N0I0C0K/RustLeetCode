use std::collections::{HashSet, VecDeque};

struct Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        let a1 = Solution::_max_opera(&nums, nums[0] + nums[l - 1]);
        let a2 = Solution::_max_opera(&nums, nums[0] + nums[1]);
        let a3 = Solution::_max_opera(&nums, nums[l - 2] + nums[l - 1]);
        return a1.max(a2).max(a3);
    }

    fn _max_opera(nums: &Vec<i32>, n: i32) -> i32 {
        let mut dp: HashSet<(usize, usize)> = HashSet::new();
        let mut que: VecDeque<(usize, usize)> = VecDeque::new();

        let l = nums.len();

        que.push_back((0, l - 1));
        let mut res = 0;
        while let Some((s, e)) = que.pop_front() {
            if s >= e {
                return (l / 2) as i32;
            }
            if dp.contains(&(s, e)) {
                continue;
            }
            dp.insert((s, e));
            res = res.max((l - (e - s + 1)) / 2);
            if nums[s] + nums[e] == n {
                que.push_back((s + 1, e - 1));
            }
            if s < e - 1 && nums[s] + nums[s + 1] == n {
                que.push_back((s + 2, e));
            }
            if e > s + 1 && nums[e] + nums[e - 1] == n {
                que.push_back((s, e - 2));
            }
        }

        res as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let nums = vec![3, 2, 1, 2, 3, 4];
        let res = Solution::max_operations(nums);
        assert_eq!(res, 3);
    }
}
