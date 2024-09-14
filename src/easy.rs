struct Solution;

impl Solution {
    pub fn maximize_win(prize_positions: Vec<i32>, k: i32) -> i32 {
        let len = prize_positions.len();
        if len == 1 {
            return 1;
        }
        let lower_bound = |x: i32| -> usize {
            let mut l = 0;
            let mut r = len;
            while l < r {
                let m = l + (r - l) / 2;
                if prize_positions[m] <= x {
                    l = m;
                } else {
                    r = m;
                }
                if r - l == 1 {
                    break;
                }
            }
            l
        };

        let upper_bound = |x: i32| -> usize {
            let mut l = 0;
            let mut r = len;
            while l < r {
                let m = l + (r - l) / 2;
                if prize_positions[m] >= x {
                    r = m;
                } else {
                    l = m;
                }
                if r - l == 1 {
                    break;
                }
            }
            if prize_positions[l] >= x {
                l
            } else {
                r
            }
        };

        let mut lmax = vec![0; len];
        for i in 0..len {
            let t = upper_bound(prize_positions[i] - k);
            lmax[i] = (i - t + 1) as i32;
            if i > 0 {
                lmax[i] = lmax[i].max(lmax[i - 1]);
            }
        }

        let mut rmax = vec![0; len];
        for i in (0..len).rev() {
            let t = lower_bound(prize_positions[i] + k);
            rmax[i] = (t + 1 - i) as i32;
            if i < len - 1 {
                rmax[i] = rmax[i].max(rmax[i + 1]);
            }
        }

        (0..len - 1)
            .map(|idx| lmax[idx] + rmax[idx + 1])
            .max()
            .unwrap()
    }
}
#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::maximize_win(vec![1, 1, 2, 2, 3, 3, 5], 2), 7);
    }
}
