pub struct S217;

impl S217 {
    pub fn contains_duplicate_hs(nums: Vec<i32>) -> bool {
        let mut hs: std::collections::HashSet<i32> =
            std::collections::HashSet::with_capacity(nums.len());
        for n in nums {
            if !hs.insert(n) {
                return true;
            }
        }
        false
    }

    pub fn contains_duplicate_hm(nums: Vec<i32>) -> bool {
        let mut hm: std::collections::HashMap<i32, i32> =
            std::collections::HashMap::with_capacity(nums.len());
        for n in nums {
            if core::option::Option::is_some(&hm.insert(n, 0)) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_with_hs() {
        let mut result = S217::contains_duplicate_hs(vec![1, 2, 3, 4]);
        assert_eq!(result, false);
        result = S217::contains_duplicate_hs(vec![1, 2, 2, 4]);
        assert_eq!(result, true);
        result = S217::contains_duplicate_hs(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]);
        assert_eq!(result, true);
    }

    #[test]
    fn it_works_with_hm() {
        let mut result = S217::contains_duplicate_hm(vec![1, 2, 3, 4]);
        assert_eq!(result, false);
        result = S217::contains_duplicate_hm(vec![1, 2, 2, 4]);
        assert_eq!(result, true);
        result = S217::contains_duplicate_hm(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]);
        assert_eq!(result, true);
    }
}
