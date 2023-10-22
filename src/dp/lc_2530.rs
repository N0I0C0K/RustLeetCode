use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        let mut heap = nums.iter().map(|x| *x).collect::<BinaryHeap<i32>>();
        let mut res = 0i64;
        (0..k).for_each(|_| {
            if let Some(x) = heap.pop() {
                res += x as i64;
                let mut t = x / 3;
                if x % 3 != 0 {
                    t += 1;
                }
                heap.push(t);
            }
        });

        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let nums = vec![10, 10, 10, 10, 10];
        let res = Solution::max_kelements(nums, 5);
        assert_eq!(res, 50);
    }
}
