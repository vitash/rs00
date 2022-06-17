#![allow(unused_variables, dead_code, unused_imports)]
// #![feature(specialization)]
#![feature(async_closure, trace_macros, type_alias_impl_trait)]
// #![feature(try_trait)]
#![feature(label_break_value)]

use std::io::Read;
mod codewars;
mod control_flow;
mod err0;
mod file0;
mod fn0;
mod futrue1;
pub mod generic0;
mod internal_mut;
mod iter0;
mod jpgz;
mod leet_code;
mod lib00;
mod lifetime0;
mod macros;
mod scoping;
mod std0;
mod std_lib;
mod std_ops;
mod struct0;
mod thread0;
mod trait0;
mod trait_generic;

fn main() {
    // fn0::test();
    lifetime0::struct_move::test2();
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

#[test]
fn test_stdin() {
    let line = &mut String::new();
    std::io::stdin().read_line(line).unwrap();
    println!("{line}, {}, {:?}", line.len(), line.as_bytes()); // 会多出 \r\n
    println!("num: {:?}", line.trim().parse::<i32>()); // parse 不能有空白符
    let num = line.trim().parse::<i32>().map(|x| x.cmp(&2));
    std::io::stdin().read_line(line).expect("msg"); // 会拼接在后面
    println!("{line}, {}, {:?}", line.len(), line.as_bytes());
    // line.trim()
    _ = std::any::type_name::<u32>();
    // let val: u64 = "2930482035982309".parse().unwrap();
}
