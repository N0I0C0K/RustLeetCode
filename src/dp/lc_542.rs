use std::str::FromStr;

struct Solution;

impl Solution {
    pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
        let v = 1_000_000_000f32 / length as f32 / width as f32 / height as f32;
        let bu = v <= 1f32 || length >= 1_000_0 || width >= 1_000_0 || height >= 1_000_0;
        let hea = mass >= 100;

        if bu && hea {
            return String::from("Both");
        } else if bu {
            return String::from("Bulky");
        } else if hea {
            return String::from("Heavy");
        } else {
            return String::from("Neither");
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let res = Solution::categorize_box(2909, 3968, 3272, 727);
        println!("{res}");
    }
}
