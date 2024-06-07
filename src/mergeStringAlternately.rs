// you are given two strings word1 and word2. Merge the string bby adding letters in alternating order, string with word1. if a string is longer than other, append the additional letters onto the end of the merged string.

   pub fn merge_alternately(word1: String, word2: String) -> String{
        let mut result = String::new();
        let mut i = 0;
        let mut j = 0;

        while i < word1.len() && j < word2.len() {
            result.push(word1.chars().nth(i).unwrap());
            result.push(word2.chars().nth(j).unwrap());

            i += 1;
            j += 1;
        }
        while i < word1.len() {
            result.push(word1.chars().nth(i).unwrap());
            i += 1;
        }
        while i < word2.len() {
            result.push(word2.chars().nth(j).unwrap());
            j += 1;
        }
        result
    }
