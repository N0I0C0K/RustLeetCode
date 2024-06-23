struct Solution;

impl Solution {
    pub fn smallest_beautiful_string(s: String, k: i32) -> String {
        use std::collections::HashSet;
        let mut chs = s.chars().collect::<Vec<char>>();
        let l = chs.len();
        let a = 'a' as i32;
        let mut set: HashSet<u8> = HashSet::with_capacity(10);
        let mut find_idx: Option<usize> = None;
        for i in (0..l).rev() {
            let pos_c = chs[i];

            set.insert(pos_c as u8);

            if i >= 1 {
                set.insert(chs[i - 1] as u8);
            }
            if i >= 2 {
                set.insert(chs[i - 2] as u8);
            }
            let mut res: u8 = u8::MAX;
            let min_c = pos_c as u8 + 1;
            for c in set.iter() {
                if (c + 1) as i32 - a + 1 <= k
                    && !set.contains(&(c + 1))
                    && c + 1 < res
                    && c + 1 >= min_c
                {
                    res = c + 1;
                }
            }
            if res as i32 - a + 1 <= k {
                chs[i] = res as char;
                find_idx = Some(i);
                break;
            }
            set.clear();
        }
        if let Some(idx) = find_idx {
            for i in (idx + 1)..l {
                let mut min = u8::MAX;
                let mut max = 0_u8;
                if i >= 1 {
                    min = min.min(chs[i - 1] as u8);
                    max = max.max(chs[i - 1] as u8);
                }
                if i >= 2 {
                    min = min.min(chs[i - 2] as u8);
                    max = max.max(chs[i - 2] as u8);
                }
                let res = if min > 97 {
                    'a'
                } else if max > min + 1 {
                    (min + 1) as char
                } else {
                    'c'
                };
                chs[i] = res;
            }
        }

        return if find_idx.is_some() {
            chs.iter().collect::<String>()
        } else {
            String::from("")
        };
    }
}

mod test {
    use super::Solution;

    #[test]
    fn test() {
        let s = String::from("abcz");
        let res = Solution::smallest_beautiful_string(s, 26);
        assert_eq!(res, "abda");
    }
    #[test]
    fn test1() {
        let s = String::from("abdc");
        let res = Solution::smallest_beautiful_string(s, 4);
        assert_eq!(res, "acba");
    }
}
