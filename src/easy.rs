struct Solution;

impl Solution {
    fn get_sum_by_val(val: i64, x: i32) -> i64 {
        let mut res = 0_i64;
        for i in 0..64 {
            if (1 << i) > val {
                break;
            }
            if (i + 1) % x == 0 {
                let k = 1 << (i + 1);
                res += (val + 1) / k * k / 2;
                let m = (val + 1) % k - k / 2;
                res += m.max(0);
            }
        }
        res
    }

    pub fn find_maximum_number(k: i64, x: i32) -> i64 {
        let mut l = 0_i64;
        let mut r = 1e3 as i64;
        while l < r {
            let mid = l + (r - l) / 2;
            let t = Solution::get_sum_by_val(mid, x);
            if r - l == 1 {
                break;
            }
            if t > k {
                r = mid;
            } else {
                l = mid;
            }
        }
        l
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_get_sum() {
        assert_eq!(Solution::get_sum_by_val(6, 1), 9);
        assert_eq!(Solution::get_sum_by_val(7, 1), 12);
        assert_eq!(Solution::get_sum_by_val(6, 2), 3)
    }

    #[test]
    fn test() {
        let sol = Solution::find_maximum_number(9, 1);
        assert_eq!(sol, 6);
    }

    #[test]
    fn test1() {
        let sol = Solution::find_maximum_number(7, 2);
        assert_eq!(sol, 9);
    }
}
