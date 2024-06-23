struct Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let chs = word
            .chars()
            .enumerate()
            .map(|(idx, c)| if c.is_uppercase() { Some(idx) } else { None })
            .filter(|x| x.is_some())
            .collect::<Vec<Option<usize>>>();

        return if chs.len() == 1 {
            if chs[0] == Some(0) {
                true
            } else {
                false
            }
        } else if chs.len() == 0 {
            true
        } else if chs.len() == word.len() {
            true
        } else {
            false
        };
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
}
