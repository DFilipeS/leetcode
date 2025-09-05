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
    add_digits(l1, l2, 0)
}

fn add_digits(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    carry: i32,
) -> Option<Box<ListNode>> {
    if l1.is_none() && l2.is_none() && carry != 0 {
        return Some(Box::new(ListNode::new(carry)));
    }

    if l1.is_none() && l2.is_none() {
        return None;
    }

    let node1 = l1.map_or(ListNode::new(0), |node| *node);
    let node2 = l2.map_or(ListNode::new(0), |node| *node);

    let sum = node1.val + node2.val + carry;
    let digit = sum % 10;
    let carry = sum / 10;

    let mut result = ListNode::new(digit);
    result.next = add_digits(node1.next, node2.next, carry);

    return Some(Box::new(result));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        let cases = vec![
            (vec![2, 4, 3], vec![5, 6, 4], vec![7, 0, 8]),
            (vec![0], vec![0], vec![0]),
            (
                vec![9, 9, 9, 9, 9, 9, 9],
                vec![9, 9, 9, 9],
                vec![8, 9, 9, 9, 0, 0, 0, 1],
            ),
        ];

        for (n1, n2, expected) in cases {
            let l1 = vec_to_list_nodes(n1);
            let l2 = vec_to_list_nodes(n2);
            let result = add_two_numbers(l1, l2);

            if !is_equal(result.clone(), vec_to_list_nodes(expected.clone())) {
                panic!(
                    "expected {:?}, but got {:?}",
                    expected,
                    list_nodes_to_vec(&result)
                );
            }
        }
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

    fn is_equal(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> bool {
        let mut p1 = &l1;
        let mut p2 = &l2;

        while p1.is_some() && p2.is_some() {
            let node1 = p1.as_ref().unwrap();
            let node2 = p2.as_ref().unwrap();

            if node1.val != node2.val {
                return false;
            }

            p1 = &node1.next;
            p2 = &node2.next;
        }

        p1.is_none() && p2.is_none()
    }

    fn list_nodes_to_vec(list: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = vec![];
        let mut list = list;

        while list.is_some() {
            let node = list.as_ref().unwrap();
            result.push(node.val);
            list = &node.next;
        }

        result
    }
}
