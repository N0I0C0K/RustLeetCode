struct Solution;

impl Solution {
    pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
        return words
            .iter()
            .map(|x| {
                x.split(separator)
                    .filter(|t| t.len() > 0)
                    .map(|t| String::from(t))
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>()
            .concat();
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test1() {}
}
