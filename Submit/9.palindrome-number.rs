/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x.is_negative() {
            return false;
        }
        
        let numstr = x.to_string();
        let numrev = numstr.chars().rev();

        for (i, v) in numrev.enumerate() {
            let front = numstr.chars().nth(i).unwrap();

            if front != v { return false }
        }

        true
    }
}
// @lc code=end

