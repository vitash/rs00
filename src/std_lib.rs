use std::collections::{HashMap, HashSet};
use std::slice::Iter;
use std::any::Any;

#[test]
fn test() {
    hashmap1();
}

fn hashmap1() {
    let hm = &mut HashMap::<String, &str>::new();
    let res = hm.insert("sds".to_string(), "dd");
    hm.get("k"); // key 可以是 &str
    "".len();

    let v1 = &mut vec![1, 2, 3].iter();
}

fn hashset1() {
    let hs1: &HashSet<&u32> = &[1, 2_u32].iter().collect();
    let hs2: &HashSet<u32> = &[1, 2_u32].iter().cloned().collect();
    let hs = &mut (0..6_u32).collect::<HashSet<_>>();
    let r = hs.insert(3);
    let r = hs.difference(&[1, 2_u32].iter().cloned().collect::<HashSet<_>>());

    let s1: Option<String> = vec!["ds".to_string(), "ds".to_string()].into_iter().next();
    // let it1: impl Iterator<Item = &i32> = vec![1, 2, 3].iter();
    let v1 = vec![1, 2, 3];
    let it1: &mut Iter<'_, i32> = &mut v1.iter(); // impl Iterator<Item = &'a T> for Iter<'a, T>
    let a: Option<&i32> = it1.next();
    let a: Option<i32> = it1.cloned().next();
}

use std::mem::transmute;
use std::mem::size_of;

fn main() {

    println!("{:?}", size_of::<&[i32; 3]>());
    println!("{:?}", size_of::<&[i32]>());

    let v : [i32; 5] = [1,2,3,4,5];
    let p : &[i32] = &v[2..4];
    unsafe {
        let (ptr, len) : (usize, isize) = transmute(p);
        println!("{} {}", ptr, len);

        let ptr = ptr as *const i32;
        for i in 0..len {
            println!("{}", *ptr.offset(i));
        }
    }
}

// struct Any1;
// fn test_any1() -> Box<Any> {
//     3 as Box<Any>
// }