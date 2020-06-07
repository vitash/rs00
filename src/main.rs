#![allow(unused_variables)]
#![allow(dead_code)]

mod contral_flow;
mod iter0;
mod leet_code;
mod scoping;
mod fn0;

fn main() {
    fn0::test();
    let a = 3;
    let b = "ds";
    println!("__________________________________________");
}

#[derive(Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    fn del_next(&mut self) {
        // let mut n1 = self.next.take();
        // let n2 = n1.take().unwrap().next;
        // self.next = n2;
        if let Some(next) = self.next.take() {
            self.next = next.next;
        }
    }
    fn insert(&mut self, val: i32) {
        let mut node = ListNode::new(val);
        node.next = self.next.take();
        self.next = Some(Box::new(node));
    }
}

fn fn1(v1: &mut Vec<i32>, v2: Vec<i32>) {
    let mut a1 = v2;
    a1.push(3);
}