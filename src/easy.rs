struct Solution;

impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        let n = people.len();
        people.sort_unstable_by(|a, b| b.cmp(a));
        let mut l = 0_usize;
        let mut r = n - 1;
        let mut res = 0;
        while l < r && l < n {
            if people[l] + people[r] > limit {
                l += 1;
                res += 1;
                continue;
            }
            res += 1;
            l += 1;
            r -= 1;
        }
        if l == r {
            res += 1;
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        let people = vec![1, 2];
        let res = Solution::num_rescue_boats(people, 3);
        assert_eq!(res, 1)
    }
    #[test]
    fn test2() {
        let people = vec![3, 2, 2, 1];
        let res = Solution::num_rescue_boats(people, 3);
        assert_eq!(res, 3)
    }
}
