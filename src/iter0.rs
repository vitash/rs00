#[test]
pub fn test1() {
    str1();
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

fn fn4() {
    let it1 = 1..3;
    let (a1, b1): (Vec<i32>, Vec<i32>) = it1.partition(|&x| x > 2);
    
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

fn vec1() {
    let v1 = vec![1, 3, 3, 4];
    let v2 = &mut [&1, &3, &4];
    v2[0] = &3;
    assert_eq!(&[&3, &3, &4], v2);
}

fn str1() {
    let s1 = " sd dd ".to_string();
    let s2 = s1.trim_matches(' ');
    assert_ne!(&*s1, s2); // s2 只是切片

    let s3 = String::new();
    let s4 = s3.trim_matches(&[' ', ','][..]);
}
