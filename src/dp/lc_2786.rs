struct Solution;

impl Solution {
    pub fn max_score(nums: Vec<i32>, x: i32) -> i64 {
        let x = x as i64;
        let n = nums.len();
        let mut even_max = (0_i64, 0_usize);
        let mut ood_max = (0_i64, 0_usize);
        if nums[0] % 2 == 0 {
            even_max.0 = nums[0] as i64;
            ood_max.0 = -1_000_000;
        } else {
            ood_max.0 = nums[0] as i64;
            even_max.0 = -1_000_000;
        }

        for (i, num) in nums.iter().enumerate().skip(1) {
            let num = *num as i64;
            if num % 2 == 0 {
                let t = (ood_max.0 + num as i64 - x).max(even_max.0 + num);
                if t > even_max.0 {
                    even_max = (t, i);
                }
            } else {
                let t = (ood_max.0 + num as i64).max(even_max.0 + num - x);
                if t > ood_max.0 {
                    ood_max = (t, i);
                }
            }
        }
        even_max.0.max(ood_max.0)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        let nums = vec![8, 50, 65, 85, 8, 73, 55, 50, 29, 95, 5, 68, 52, 79];
        let res = Solution::max_score(nums, 74);
        assert_eq!(res, 470);
    }
}
