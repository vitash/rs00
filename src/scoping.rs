struct Point {
    x: u8,
    y: u8,
}

pub fn test() {}

fn fn1() {
    let p1 = Point { x: 3, y: 4 };
    let Point { ref x, .. } = p1;
    let b = x;
    println!("{}", b == &3);
}
