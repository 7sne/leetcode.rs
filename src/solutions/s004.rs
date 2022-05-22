struct S004;

impl S004 {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut merged = [nums1, nums2].concat();
        let len = merged.len();
        merged.sort();
        let median = if len % 2 == 0 {
            (merged[len / 2] as f64 + merged[len / 2 - 1] as f64) / 2.0
        } else {
            merged[len / 2] as f64
        };
        median
    }
}
