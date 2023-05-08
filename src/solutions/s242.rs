pub struct S242;

impl S242 {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut hm: std::collections::HashMap<u8, i32> = std::collections::HashMap::with_capacity(s.chars().count());
        s.bytes().for_each(|ch| {
            hm.entry(ch).and_modify(|v| *v += 1).or_insert(1);
        });
        t.bytes().for_each(|ch| {
            hm.entry(ch).and_modify(|v| *v -= 1);
        });
        hm.values().all(|v| *v == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut result = S242::is_anagram("anagram".to_string(), "nagaram".to_string());
        assert_eq!(result, true);
        result = S242::is_anagram("rat".to_string(), "car".to_string());
        assert_eq!(result, false);
        result = S242::is_anagram("ab".to_string(), "a".to_string());
        assert_eq!(result, false);
        result = S242::is_anagram("aacc".to_string(), "cacc".to_string());
        assert_eq!(result, false);
    }
}
