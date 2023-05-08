pub struct S001;

impl S001 {
    #[allow(dead_code)]
    pub(crate) fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hm: std::collections::HashMap<i32, usize> = std::collections::HashMap::with_capacity(nums.len());
        for (i, n) in nums.into_iter().enumerate() {
            let maybe_match = hm.get(&(target - n));
            if maybe_match.is_some() {
                return vec![*maybe_match.unwrap() as i32, i as i32];
            }
            hm.insert(n, i);
        }
        panic!("Solution always exists");
    }
}

// 2
// 7
// 11
// 15

// 13

// [2]

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works() {
        let solution = S001::two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(solution, vec![0, 1]);
        let solution = S001::two_sum(vec![3, 2, 4], 6);
        assert_eq!(solution, vec![1, 2]);
    }
}
