// Definition for singly-linked list.
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

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        let cases = vec![vec![2, 4, 3]];

        for case in cases {
            let l1 = vec_to_list_nodes(case);
            dbg!(l1);
            // TODO: Call actual function with `current`.
        }
        todo!();
    }

    fn vec_to_list_nodes(nums: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current = None;
        for &val in nums.iter().rev() {
            let mut node = Box::new(ListNode::new(val));
            node.next = current;
            current = Some(node);
        }
        current
    }
}
