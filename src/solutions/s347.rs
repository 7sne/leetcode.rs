use std::{collections::HashMap, vec};

pub struct S347;

impl S347 {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut hm: std::collections::HashMap<i32, i32> = std::collections::HashMap::with_capacity(nums.len());
        for n in nums {
            hm.entry(n).and_modify(|v| *v += 1).or_insert(1);
        }
        let mut v: Vec<(i32, i32)> = hm.into_iter().collect();
        v.sort_by(|a, b| b.1.cmp(&a.1));
        v.into_iter().take(k as usize).map(|(n, _)| n).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(S347::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
        assert_eq!(S347::top_k_frequent(vec![1], 1), vec![1]);
        assert_eq!(S347::top_k_frequent(vec![1, 2], 2), vec![1, 2]);
        assert_eq!(S347::top_k_frequent(vec![-1, -1], 2), vec![-1]);
        assert_eq!(S347::top_k_frequent(vec![4, 1, -1, 2, -1, 2, 3], 2), vec![-1, 2]);
    }
}
