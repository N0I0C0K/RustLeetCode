struct Solution;

impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut digits = num
            .to_string()
            .chars()
            .enumerate()
            .collect::<Vec<(usize, char)>>();
        digits.sort_by_key(|x| (x.1 as u8, x.0));
        let mut res = num.to_string().chars().collect::<Vec<char>>();
        let mut t = '0';
        let mut m = '0';
        for i in 0..digits.len() {
            if res[i] != digits[digits.len() - i - 1].1 {
                //res.swap(i, digits[digits.len() - i - 1].0);
                t = res[i];
                m = digits[digits.len() - i - 1].1;
                res[i] = digits[digits.len() - i - 1].1;
                break;
            }
        }
        for i in (0..digits.len()).rev() {
            if res[i] == m {
                res[i] = t;
                break;
            }
        }

        res.iter().collect::<String>().parse().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        let res = Solution::maximum_swap(98368);
        assert_eq!(res, 98863);
    }
}
