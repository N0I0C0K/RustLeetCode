struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        let min_idx = nums
            .iter()
            .enumerate()
            .min_by_key(|(idx, x)| x.abs())
            .unwrap()
            .0;
        let len = nums.len();
        let mut l = Some(min_idx);
        let mut r = if min_idx < len - 1 {
            Some(min_idx + 1)
        } else {
            None
        };
        let mut choose_l = false;
        while l.is_some() || r.is_some() {
            if l.is_none() {
                res.push(nums[r.unwrap()] * nums[r.unwrap()]);
                choose_l = false;
            } else if r.is_none() {
                res.push(nums[l.unwrap()] * nums[l.unwrap()]);
                choose_l = true;
            } else {
                if nums[l.unwrap()].abs() <= nums[r.unwrap()].abs() {
                    res.push(nums[l.unwrap()] * nums[l.unwrap()]);
                    choose_l = true;
                } else {
                    res.push(nums[r.unwrap()] * nums[r.unwrap()]);
                    choose_l = false;
                }
            }
            if choose_l {
                l = if l == Some(0) {
                    None
                } else {
                    Some(l.unwrap() - 1)
                }
            } else {
                r = if r == Some(len - 1) {
                    None
                } else {
                    Some(r.unwrap() + 1)
                }
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
        assert_eq!(Solution::min_end(2, 7), 15);
    }
}
