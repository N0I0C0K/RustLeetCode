struct Solution;
use std::collections::HashSet;
impl Solution {
    fn dfs(
        child: &Vec<Vec<usize>>,
        nums: &Vec<i32>,
        set: &mut HashSet<i32>,
        vis: &mut Vec<bool>,
        x: usize,
    ) {
        if vis[x] {
            return;
        }
        vis[x] = true;
        set.insert(nums[x]);

        for chi in child[x].iter() {
            Self::dfs(child, nums, set, vis, *chi);
        }
    }

    pub fn smallest_missing_value_subtree(parents: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let mut dp = vec![1; parents.len()];
        let mut child: Vec<Vec<usize>> = vec![vec![]; parents.len()];

        for (idx, x) in parents.iter().enumerate().skip(1) {
            child[*x as usize].push(idx);
        }

        let mut tnode = 1;
        let mut tidx = None;

        for (idx, x) in nums.iter().enumerate() {
            if *x == 1 {
                tidx = Some(idx);
                break;
            }
        }
        let mut vis = vec![false; parents.len()];
        let mut set: HashSet<i32> = HashSet::new();

        while let Some(idx) = tidx {
            Self::dfs(&child, &nums, &mut set, &mut vis, idx);

            while set.contains(&tnode) {
                tnode += 1;
            }
            dp[idx] = tnode;
            if parents[idx] != -1 {
                tidx = Some(parents[idx] as usize);
            } else {
                tidx = None;
            }
        }

        dp
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        let parents = vec![-1, 0, 0, 2];
        let nums = vec![1, 2, 3, 4];

        let res = Solution::smallest_missing_value_subtree(parents, nums);
        assert_eq!(res, [5, 1, 1, 1]);
    }
}
