// lc_1483

struct Solution;

struct TreeAncestor {
    parent_with_floor: Vec<[i32; 17]>,
    parent: Vec<i32>,
    node_len: usize,
    depth: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TreeAncestor {
    fn _high_bit(mut n: i32) -> i32 {
        for i in 0..17 {
            if n < (1 << i) {
                return i - 1;
            }
        }
        panic!()
    }
    fn new(n: i32, parent: Vec<i32>) -> Self {
        let mut parent_with_floor = vec![[-1; 17]; parent.len()];
        let depth = vec![0; parent.len()];

        let mut res = TreeAncestor {
            parent_with_floor: parent_with_floor,
            node_len: parent.len(),
            parent,
            depth,
        };
        res.build();
        return res;
    }

    fn build(&mut self) {
        use std::collections::VecDeque;
        let mut root_node = 0;

        let mut children = vec![vec![]; self.parent.len()];

        for (idx, node) in self.parent.iter().enumerate() {
            if *node == -1 {
                continue;
            }
            children[*node as usize].push(idx as i32)
        }
        let mut que: VecDeque<i32> = VecDeque::new();
        self.parent_with_floor[0][0] = 0;
        que.push_back(root_node);
        while let Some(node) = que.pop_front() {
            let cur_parent = self.parent[node as usize];
            self.parent_with_floor[node as usize][0] = node;
            if cur_parent != -1 {
                self.depth[node as usize] = self.depth[cur_parent as usize] + 1;
                for i in 1..17 {
                    let t = self.get_kth_ancestor(cur_parent, (1 << (i - 1)) - 1);
                    if t == -1 {
                        break;
                    }
                    self.parent_with_floor[node as usize][i] = t;
                }
            }

            for child in children[node as usize].iter() {
                que.push_back(*child);
            }
        }
    }

    fn get_kth_ancestor(&self, node: i32, k: i32) -> i32 {
        if k > self.depth[node as usize] {
            return -1;
        }
        if k == 0 {
            return node;
        }
        let t = TreeAncestor::_high_bit(k);
        if t > 16 {
            return -1;
        }
        if k == (1 << t) {
            return self.parent_with_floor[node as usize][(t + 1) as usize];
        }
        return self.get_kth_ancestor(
            self.parent_with_floor[node as usize][(t + 1) as usize],
            k - (1 << t),
        );
    }
}

/**
 * Your TreeAncestor object will be instantiated and called as such:
 * let obj = TreeAncestor::new(n, parent);
 * let ret_1: i32 = obj.get_kth_ancestor(node, k);
 */

#[cfg(test)]
mod test {
    use super::{Solution, TreeAncestor};

    #[test]
    fn test() {
        let a = TreeAncestor::new(7, vec![-1, 0, 0, 1, 1, 2, 2]);
        assert_eq!(a.get_kth_ancestor(3, 1), 1);
    }
}
