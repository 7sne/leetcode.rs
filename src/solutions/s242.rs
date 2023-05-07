pub struct S242;

fn chars_to_hm(s: String) -> std::collections::HashMap<char, i32> {
    let chs = s.chars();
    let mut hm: std::collections::HashMap<char, i32> =
        std::collections::HashMap::with_capacity(chs.clone().count());
    chs.for_each(|ch| {
        hm.entry(ch).and_modify(|v| *v += 1).or_insert(1);
    });
    hm
}

impl S242 {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut hm = chars_to_hm(s);
        for ch in t.chars() {
            hm.entry(ch).and_modify(|v| {
                *v -= 1;
            });
        }
        hm.into_values().all(|v| v == 0)
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
