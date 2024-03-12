// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
  val: i32,
  next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl Solution {
    pub fn remove_zero_sum_sublists(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Move list values into an array
        let mut list = vec![];
        while let Some(node) = head {
            list.push(node.val);
            head = node.next;
        }

        // Remove zero-sum sequences from array
        let mut start = 0;
        while start < list.len() {
            let mut prefix = 0;
            let mut end = start;
            while end < list.len() {
                prefix += list[end];
                if prefix == 0 {
                    list.drain(start..=end);
                    start -= 1;
                    break;
                }
                end += 1;
            }
            start += 1;
        }

        // Create new list
        let mut new = ListNode::new(0);
        let mut n = &mut new;
        for val in list.into_iter().map(ListNode::new).map(Box::new) {
            n.next = Some(val);
            n = n.next.as_mut()?;
        }
        n.next = None;
        new.next
    }
}

struct Solution;

fn main() {
    // Create a sample linked list with zero-sum sublists
    let head = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: -3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None }))
                    }))
                }))
            }))
        }))
    }));

    // Print the original linked list
    println!("Original list: ");
    let mut current = &head;
    while let Some(node) = current {
        println!("{}", node.val);
        current = &node.next;
    }

    // Remove zero-sum sublists
    let new_head = Solution::remove_zero_sum_sublists(head);

    // Print the resulting linked list
    println!("List after removing zero-sum sublists: ");
    let mut current = &new_head;
    while let Some(node) = current {
        println!("{}", node.val);
        current = &node.next;
    }
}