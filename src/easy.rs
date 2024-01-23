struct Solution;

impl Solution {
    pub fn alternating_subarray(nums: Vec<i32>) -> i32 {
        let mut res = -1;
        let (mut l, mut r) = (0, 1);
        while r < nums.len() {
            if r - l == 1 && nums[r] - nums[l] != 1 {
                r += 1;
                l += 1;
                continue;
            }
            if r - l >= 2 && nums[r] != nums[r - 2] {
                l = r - 1;
                continue;
            }
            res = res.max((r - l + 1) as i32);
            r += 1;
        }
        return res;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        let res = Solution::alternating_subarray(vec![14, 30, 29, 49, 3, 23, 44, 21, 26, 52]);
        assert_eq!(res, -1);
    }
}
