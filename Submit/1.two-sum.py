#
# @lc app=leetcode id=1 lang=python3
#
# [1] Two Sum
#


# @lc code=start
class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        map = {}

        for i, v in enumerate(nums):
            search = map.get((target - v), None)
            if search is not None:
                return [search, i]

            map[v] = i


# @lc code=end
