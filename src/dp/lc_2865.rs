struct Solution;

impl Solution {
    pub fn maximum_sum_of_heights(max_heights: Vec<i32>) -> i64 {
        let n = max_heights.len();
        let mut pfix: Vec<i64> = vec![0; n];
        pfix[0] = max_heights[0] as i64;
        for i in 1..n {
            pfix[i] = max_heights[i] as i64;
            let mut f = false;
            for k in (0..i).rev() {
                if max_heights[k] <= max_heights[i] {
                    pfix[i] = pfix[i].max(pfix[k] + max_heights[i] as i64 * (i - k) as i64);
                    f = true;
                    break;
                }
            }
            if !f {
                pfix[i] *= (i + 1) as i64;
            }
        }

        let mut efix: Vec<i64> = vec![0; n];
        efix[n - 1] = max_heights[n - 1] as i64;

        for i in (0..(n - 1)).rev() {
            efix[i] = max_heights[i] as i64;
            let mut f = false;
            for k in (i + 1)..n {
                if max_heights[k] <= max_heights[i] {
                    efix[i] = efix[i].max(efix[k] + max_heights[i] as i64 * (k - i) as i64);
                    f = true;
                    break;
                }
            }
            if !f {
                efix[i] *= (n - i) as i64;
            }
        }

        (0..n)
            .map(|x| pfix[x] + efix[x] - max_heights[x] as i64)
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        let res = Solution::maximum_sum_of_heights(vec![5, 3, 4, 1, 1]);
        assert_eq!(res, 13);
    }
}
