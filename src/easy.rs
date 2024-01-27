struct Solution;

impl Solution {
    pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter()
            .enumerate()
            .map(|(idx, x)| match idx.count_ones() {
                p if p as i32 == k => *x,
                _ => 0,
            })
            .sum::<i32>()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {}
}
