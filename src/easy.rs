struct Solution;

impl Solution {
    fn check_c(chs: &Vec<char>, k: i32, t: char) -> i32 {
        let (mut l, mut r) = (0, 0);
        let mut res: i32 = 0;
        let mut cnt: i32 = 0;
        while r < chs.len() {
            if chs[r] == 'F' {
                cnt += 1;
            }
            while cnt > k && l < r {
                if chs[l] == t {
                    cnt -= 1;
                }
                l += 1;
            }
            r += 1;
            res = res.max((r - l) as _);
        }

        return res;
    }

    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        //use std::collections::VecDeque;
        let chs = answer_key.chars().collect::<Vec<char>>();
        return Solution::check_c(&chs, k, 'T').max(Solution::check_c(&chs, k, 'F'));
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
