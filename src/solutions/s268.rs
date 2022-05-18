pub struct S268;

impl S268 {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        Self::im_mut(nums)
    }
    pub fn im_mut(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut missing = 0;
        nums.windows(2).for_each(|window| {
            if window[0] + 1 != window[1] {
                missing = window[1] - 1;
            }
        });
        missing
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = S268::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]);
        assert_eq!(result, 8);
    }
}
