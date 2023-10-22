use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn bfs(n: usize, edges: &Vec<Vec<i32>>, vis: &mut Vec<bool>, idx: usize) -> i64 {
        if vis[idx] {
            return 0;
        }
        let mut que = VecDeque::<usize>::new();
        que.push_back(idx);

        vis[idx] = true;

        let mut nums = 0i64;

        while let Some(top) = que.pop_front() {
            nums += 1;
            for v in edges[top].iter() {
                if !vis[*v as usize] {
                    que.push_back(*v as usize);
                    vis[*v as usize] = true;
                }
            }
        }

        nums
    }

    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;
        let mut vis = vec![false; n];
        let mut map: Vec<Vec<i32>> = vec![vec![]; n];

        for t_pair in edges.iter() {
            map[t_pair[0] as usize].push(t_pair[1]);
            map[t_pair[1] as usize].push(t_pair[0]);
        }

        let mut all = 0i64;
        let mut list = vec![];
        for i in 0..n {
            let t = Self::bfs(n, &map, &mut vis, i);
            if t > 0 {
                list.push(t);
                all += t;
            }
        }

        let mut res = 0;

        for t in list.iter() {
            res += t * (all - t);
        }

        res / 2
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let edges = vec![vec![0, 1], vec![0, 2], vec![1, 2]];
        let res = Solution::count_pairs(3, edges);
        println!("{res}");
    }
}
