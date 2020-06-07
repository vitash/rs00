#[derive(Debug)]
struct Point {
    x: u8,
    y: u8,
}

pub fn test() {
    fn3();
}

struct Box1<T>(T);
fn fn1() {
    let b1 = Box1(3);
    let Box1(b) = b1;

    let p1 = Point { x: 3, y: 4 };
    let Point { ref x, .. } = p1;
    let b2 = Some(Box::new(1));
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

fn fn3() {
    let p1 = Point { x: 3, y: 33 };
    // let p2: &Point = p1;    // expected `&scoping::Point`, found struct `scoping::Point`
    // fn3_1(p2);
    fn3_1(p1);
}

fn fn3_1(ref p1: Point) {
    let a = p1;
    println!("{:?}", a);
}

fn mut2(p1: Point, p2: Point) {
    let mut a = Point { x: 1, y: 3 };
    // let mut a2 = a;
    let b = &mut a;
    // a.x += 3; // 错误，a 已失效
    b.x += 4;
    // a = p1; // 错误, a 先的引用被悬挂了
    // a.x += 3; // 错误
    // println!("{:?},", a);
    println!("{:?},", b);
}

fn mut3() {
    let mut a = 3;
    a += 4;
    let mut b = a;
    let mut c = a;
    a += 42;
    b += 44;
    c += 10;
    println!("{:?}, {:?}, {:?}", a, b, c);
}
