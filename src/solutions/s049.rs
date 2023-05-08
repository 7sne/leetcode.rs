use std::vec;

pub struct S049;

impl S049 {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hm: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::with_capacity(strs.len());
        strs.into_iter().for_each(|s| {
            let mut s_chars: Vec<char> = s.chars().collect();
            s_chars.sort_unstable();
            let s_sorted: String = s_chars.into_iter().collect();
            hm.entry(s_sorted.clone())
                .and_modify(|v| v.push(s.clone()))
                .or_insert(vec![s.clone()]);
        });
        hm.values().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = S049::group_anagrams(vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ]);
        assert_eq!(
            result,
            vec![
                vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
                vec!["nat".to_string(), "tan".to_string()],
                vec!["bat".to_string()]
            ]
        );
    }
}
