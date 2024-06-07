// given a string s containing just characters '(',')','{','}','[',']', determine if the input string is valid.

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut strack = Vec::new();
        let mapping = vec![(')', '(', ']', '[', '}', '{')];
        for c in s.chars(){
            if c == '(' || c =='{' || c == '[' {
                stack.push(c);
            } else {
                if stack.is_empty(){
                    return false;
                }
                let top = stack.pop().unwrap();
                let mut found = false;
                for &(close, open) in &mapping {
                    if c == close && top == open {
                        found = true;
                        break;
                    }
                }
                if !found {
                    return false;
                }
            }
        }
        strack.is_empty()
    }
}