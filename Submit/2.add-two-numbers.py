#
# @lc app=leetcode id=2 lang=python3
#
# [2] Add Two Numbers
#


# @lc code=start
# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution:
    def addTwoNumbers(
        self, l1: Optional[ListNode], l2: Optional[ListNode]
    ) -> Optional[ListNode]:
        head = ListNode(0)
        tail = head
        carry = 0

        while l1 is not None or l2 is not None or carry != 0:
            sum = carry

            if l1 is not None:
                sum += l1.val

            if l2 is not None:
                sum += l2.val

            if sum >= 10:
                carry = 1
                tail.next = ListNode(val=(sum - 10))
            else:
                carry = 0
                tail.next = ListNode(val=sum)

            tail = tail.next

            l1 = l1.next if l1 is not None else None
            l2 = l2.next if l2 is not None else None

        return head.next


# @lc code=end
