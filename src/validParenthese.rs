// given a string s containing just characters '(',')','{','}','[',']', determine if the input string is valid.

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut strack = Vec::new();
        let mapping = vec![(')', '(', ']', '[', '}', '{')];
    }
}