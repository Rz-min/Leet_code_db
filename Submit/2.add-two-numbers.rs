/*
 * @lc app=leetcode id=2 lang=rust
 *
 * [2] Add Two Numbers
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut head;
        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let mut sum = carry;
            if let Some(l1_node) = l1 {
                sum += l1_node.val;
                l1 = l1_node.next
            }
            if let Some(l2_node) = l2 {
                sum += l2_node.val;
                l2 = l2_node.next;
            }

            carry = sum / 10;
            let stay = sum % 10;
            println!("check: carry = {}, stay = {}", carry, stay);
            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(stay)));
            tail = &mut tail.as_mut().unwrap().next;
        }
        println!("HOGE: {:?}", head);
        head.unwrap().next
    }
}
// @lc code=end
