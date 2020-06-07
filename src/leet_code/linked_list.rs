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

