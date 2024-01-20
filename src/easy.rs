struct Solution;

impl Solution {
    pub fn min_operations_max_profit(
        customers: Vec<i32>,
        boarding_cost: i32,
        running_cost: i32,
    ) -> i32 {
        if boarding_cost * 4 < running_cost {
            return -1;
        }
        let (mut res, mut tmax) = (-1, -1);
        let (mut lun, mut cur) = (0, 0);
        let mut has = 0;

        for (_, x) in customers.iter().enumerate() {
            has += *x as i32;
            let tm = has.min(4);
            cur += tm * boarding_cost;
            has -= tm;

            lun += 1;
            cur -= running_cost;

            if cur > tmax {
                (res, tmax) = (lun, cur);
            }
        }
        while has > 0 {
            let tm = has.min(4);
            cur += tm * boarding_cost;
            has -= tm;

            lun += 1;
            cur -= running_cost;

            if cur > tmax {
                (res, tmax) = (lun, cur);
            }
        }
        return res;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_1() {
        let res = Solution::min_operations_max_profit(vec![8, 3], 5, 6);
        assert_eq!(res, 3);
    }
}
