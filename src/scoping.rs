struct Point {
    x: u8,
    y: u8,
}

pub fn test() {}

fn fn1() {
    let p1 = Point { x: 3, y: 4 };
    let Point { ref x, .. } = p1;
}

fn fn2(a: i32, b: Option<i32>) {
    match a {
        ten @ 0..=10 => println!("{}", ten),
        other => println!("{}", other),
    }
    match b {
        Some(b @ 0..=20) => b,
        Some(b) if (0..2).contains(&b) => b,
        _ => 30,
    };
    // if let Some(c @ 0..3) != b { return; } // 语法和后面的语义不支持，格式化也不支持三行变成一行
    // println!("{}, c in 1..3", c);
}
