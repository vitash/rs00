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

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut curr = &mut head;
    let mut end = 0;
    while let Some(node) = curr {
        end += 1;
        curr = &mut node.next;
    }

    if n > end {
        return head;
    }
    if n == end {
        return head.unwrap().next;
    }

    curr = &mut head;
    while end > n {
        end -= 1;
        curr = &mut curr.as_mut().unwrap().next;
    }
    curr.as_mut().unwrap().next = curr.as_ref().unwrap().next.as_ref().unwrap().next.clone();
    return head;
}

// pub fn is_palindrome_234(head: Option<Box<ListNode>>) -> bool {
//     let mut head = head;
//     let mut front_node = &mut head;
//     fn check(node: &Option<Box<ListNode>>, front_node: &mut Option<Box<ListNode>>) -> bool {
//         match node {
//             None => true,
//             Some(node) => {
//                 if !check(&node.next, front_node) { return false; }
//                 else if front_node.take().unwrap().val != node.val { return false; }
//                 front_node = &mut front_node.unwrap().next;
//                 return true;
//             }
//         }
//     };

//     return check(&front_node, &mut front_node);
// }
