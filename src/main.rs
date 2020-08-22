#![allow(unused_variables, dead_code, unused_imports)]
// #![feature(specialization)]
#![feature(async_closure, trace_macros, type_alias_impl_trait)]
#![feature(try_trait)]
mod control_flow;
mod err0;
mod file0;
mod fn0;
mod futrue1;
mod generic0;
mod iter0;
mod jpgz;
mod leet_code;
mod lifetime0;
mod macros;
mod scoping;
mod serde0;
mod std0;
mod std_lib;
mod std_ops;
mod struct0;
mod thread0;
mod trait0;
mod trait_generic;

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

fn opt(a: Option<u8>, b: &u8) -> &u8 {
    b
    // let a = _.upwrap();
}
