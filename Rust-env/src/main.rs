// Leet Code Environment
pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn check() {
        println!("Link");
    }
    // 1
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

    //2
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
    //9
    pub fn is_palindrome(x: i32) -> bool {
        if x.is_negative() {
            return false;
        }

        let numstr = x.to_string();
        let numrev = numstr.chars().rev();

        for (i, v) in numrev.enumerate() {
            let front = numstr.chars().nth(i).unwrap();

            if front != v {
                return false;
            }
        }

        true
    }
}

fn main() {
    // Solution::check();
    // assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    // assert_eq!(Solution::is_palindrome(121), true);
    let node1 = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode { val: 3, next: None })),
        })),
    }));
    let node2 = Some(Box::new(ListNode {
        val: 5,
        next: Some(Box::new(ListNode {
            val: 6,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));
    assert_eq!(
        Solution::add_two_numbers(node1, node2),
        Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 8, next: None }))
            }))
        }))
    );
}
