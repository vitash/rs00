pub fn test() {
    let a = &4;
    let c = match a {
        &x @ 22 => x,
        &x @ 23..=24 => x,
        // &x @ 22 | 33 => x,  // 这就不允许了
        &x @ 123..=224 if x < 124 => x,
        1 | 2 | 3..=5 => 4,
        &b if b > 3 => b,
        _ => 1,
    };

    let ref a1 = 3;
    let ref mut a2 = 3;
    // a2 = &mut 33;

    println!("{}", a2);
}

macro_rules! else_return {
    ($opt:ident) => {
        let $opt = match $opt {
            Some(s) => s,
            None => return,
        };
    };
}
fn option1(a: Option<u8>, b: Option<u8>) {
    let a = match a {
        Some(x) => x,
        None => return,
    };
    // let b = b?;

    let c = Some(3_u8);
    else_return!(c); // 第20行做了简单的实现，只是针对 `Option` 类型，`Result` 不管它有 `?`, `try!` 做处理

    let c2 = c; // rust-analyzer，类型推断错的，实际是 u8 类型
    assert_eq!(3_u8, c2); // u8 类型，编译正确
}

// fn option2(a: Option<u8>) {
//     let a = a?;
//     // None
// }

fn loop_test() {
    for i in (0..33).step_by(2) {}
}
