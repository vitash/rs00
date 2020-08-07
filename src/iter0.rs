#[test]
pub fn test1() {
    range(3);
}

fn fn2(v1: Vec<u8>) {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    let a = v1.get(3).unwrap_or(&0);
}

use std::time;
fn fn3(r1: Result<i8, ()>) {
    let max_time = time::Instant::now();
    for _ in 0..1000 {
        println!("ds")
    }
}

#[test]
fn test_take() {
    let v1 = vec![1];
    let it2 = v1.iter().take(3);
    // v1.into_iter().
}

fn range(a1: u8) {
    for i in 0.. {
        break;
    }
    let r1 = 0_u8..;
    let r2 = 0..3;
    let r3 = ..3;
    let r31 = ..=3;
    let r4 = ..; // 这都可以

    let r1_start = r1.start;

    let r5 = 3..3;
    // <std::ops::Range::<i32> as ExactSizeIterator>::is_empty(&r5);
    // std::ops::Range::is_empty(&r5);
    // assert!(r5.is_empty());
}

// 参数的配平艺术
fn fn5(&&a1: &&u8, a2: &&u8) -> bool {
    let &a22 = a2;
    a1 == 3
}

pub fn range_float() {
    // let mut count = 0;
    // for i in (0.1..1_f32).stey_by(0.1) {} // 不可迭代
    for c in 'a'..'s' {}
}
