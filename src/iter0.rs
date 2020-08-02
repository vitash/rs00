#[test]
pub fn test1() {
    let v1: Vec<u8> = vec![];
    let res1 = v1.iter().find(|&&x| x == 3_u8);
    
    // v1.iter().max()
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

fn fn4(r2: u8) -> u8 {
    r2 * 3
}

// 参数的配平艺术
fn fn5(&&a1: &&u8, a2: &&u8) -> bool {
    let &a22 = a2;
    a1 == 3
}
