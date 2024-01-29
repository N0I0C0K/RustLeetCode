struct Solution;

impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        use std::collections::HashMap;
        let lk = key.len();
        let lr = ring.len();
        let mut dp = vec![vec![i32::MAX / 2; ring.len()]; lk];
        let mut amap: HashMap<char, Vec<usize>> = HashMap::new();
        let chars = ring.chars().collect::<Vec<char>>();
        for (idx, c) in chars.iter().enumerate() {
            amap.entry(*c)
                .and_modify(|x| x.push(idx))
                .or_insert(vec![idx]);
        }
        let get_p = |x: &usize| -> usize { *x.min(&(lr - x)) };
        let get_dis = |x1: &usize, x2: &usize| -> i32 {
            let xx1 = *x1 as i32;
            let xx2 = *x2 as i32;

            return (xx1 - xx2).abs().min((get_p(x1) + get_p(x2)) as i32);
        };
        let kchars = key.chars().collect::<Vec<char>>();
        for idx in amap.get(&kchars[0]).unwrap() {
            dp[0][*idx] = (get_dis(idx, &0) + 1) as i32;
        }

        for i in 1..lk {
            for idx in amap.get(&kchars[i]).unwrap() {
                for t_idx in amap.get(&kchars[i - 1]).unwrap() {
                    dp[i][*idx] = dp[i][*idx].min(dp[i - 1][*t_idx] + get_dis(idx, t_idx) + 1);
                }
            }
        }

        *dp[lk - 1].iter().min().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        let res = Solution::find_rotate_steps(String::from("godding"), String::from("gd"));
        assert_eq!(res, 4);
    }

    #[test]
    fn test2() {
        let res = Solution::find_rotate_steps(String::from("godding"), String::from("godding"));
        assert_eq!(res, 13);
    }
}
