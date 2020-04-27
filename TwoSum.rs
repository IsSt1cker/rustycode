use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums.len() {
            let search = target - nums[i];
            match map.get(&search) {
                Some(&result) => return vec![result, i as i32],
                None => map.insert(nums[i], i as i32),
            };
        }
        return vec![];
    }
}