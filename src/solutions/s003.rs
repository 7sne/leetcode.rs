pub struct S003;

// abc

// { "ab": 1, "a": }

// abc abc
// a -> 100100
// b -> 010010

// dvdf
// d -> 1010


// [ab]cabcbb
// a[bc]abcbb
// ab[ca]bcbb
// abc[ab]cbb

// [dv]df
// d[vd]f
// dv[df]

// a[a](bcfgh) .. 

impl S003 {
    pub(crate) fn length_of_longest_substring(s: String) -> i32 {
        let mut m = std::collections::HashMap::new();
        let mut result = 0;
        let mut last = 0;
        for (index, ch) in s.char_indices() {
            if let Some(present) = m.insert(ch, index) {
                result = result.max(index - last);
                last = last.max(present + 1);
            }
        }
        result.max(s.len() - last) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result0 = S003::length_of_longest_substring("abca".to_string());
        let result2 = S003::length_of_longest_substring("aabcfgh".to_string());
        let result1 = S003::length_of_longest_substring("pwwkew".to_string());
        let result = S003::length_of_longest_substring("dvdf".to_string());
        assert_eq!(result1, 3);
        assert_eq!(result2, 6);
        assert_eq!(result0, 3);
        assert_eq!(result, 3);
        assert_eq!(result, 3);
    }
}