pub struct S001;

impl S001 {
    #[allow(dead_code)]
    pub(crate) fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map: HashMap<i32, usize> = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            if let Some(&found) = map.get(&(target - num)) {
                return vec![found as i32, i as i32];
            }
            map.insert(*num, i);
        }
        vec![]
    }
}

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
