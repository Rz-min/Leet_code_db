#
# @lc app=leetcode id=9 lang=python3
#
# [9] Palindrome Number
#


# @lc code=start
class Solution:
    def isPalindrome(self, x: int) -> bool:
        if x < 0:
            return False

        numstr = str(x)
        numstr_rev = numstr[::-1]

        if numstr == numstr_rev:
            return True
        else:
            return False


# @lc code=end
