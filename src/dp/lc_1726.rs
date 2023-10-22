use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let l = nums.len();
        for i in 0..l {
            for j in (i + 1)..l {
                let t = nums[i] * nums[j];
                map.entry(t)
                    .and_modify(|x| {
                        *x += 1;
                    })
                    .or_insert(1);
            }
        }
        let mut res = 0;
        for i in map.values() {
            res += (i * (i - 1)) * 4;
        }
        res
    }
}

#[derive(Default, Debug)]
struct DropTest {
    tstr: String,
}

impl Drop for DropTest {
    fn drop(&mut self) {
        println!("I have drop!");
    }
}

#[cfg(test)]
mod test {
    use std::{ptr::NonNull, str::FromStr};

    use super::{DropTest, Solution};

    #[test]
    fn test1() {
        let nums = vec![2, 3, 4, 6];
        let res = Solution::tuple_same_product(nums);
        assert_eq!(res, 8);
        let nums = vec![1, 2, 4, 5, 10];
        let res = Solution::tuple_same_product(nums);
        assert_eq!(res, 16);
    }

    fn get_pointer() -> NonNull<DropTest> {
        let mut data = Box::new(DropTest::default());
        let mut pointer = NonNull::new(&mut *data).expect("null");
        std::mem::forget(data);
        return pointer;
    }

    #[test]
    fn test2() {
        let mut p = get_pointer();
        let pp = unsafe { p.as_mut() };
        (*pp).tstr.extend(['a'].iter());
        println!("{:#?}", pp);
    }
}
