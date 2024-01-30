struct Solution;

impl Solution {
    pub fn minimum_seconds(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let l = nums.len();
        let mut counter: HashMap<i32, Vec<usize>> = HashMap::new();
        for i in 0..l {
            counter
                .entry(nums[i])
                .and_modify(|x| x.push(i))
                .or_insert(vec![i]);
        }

        counter
            .keys()
            .map(|k| {
                let idxs = counter.get(&k).unwrap();
                let mut mx = idxs.first().unwrap() + l - idxs.last().unwrap();
                for i in 1..idxs.len() {
                    mx = mx.max(idxs[i] - idxs[i - 1]);
                }
                mx / 2
            })
            .min()
            .unwrap() as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        let res = Solution::minimum_seconds(vec![1, 2, 1, 2]);
        assert_eq!(res, 1);
    }
}
