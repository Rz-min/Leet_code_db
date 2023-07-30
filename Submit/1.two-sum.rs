/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        for (i, v) in nums.iter().enumerate() {
            match map.get(&(target - v)) {
                Some(&i2) => return vec![i2, i as i32],
                None => map.insert(*v, i as i32),
            };
        }
        vec![]
    }
}
// @lc code=end

