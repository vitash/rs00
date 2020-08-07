#[derive(Debug)]
struct Point {
    x: u8,
    y: u8,
}

#[test]
pub fn test() {
    pattern_slice();
}

struct Box1<T>(T);
fn fn1() {
    let b1 = Box1(3);
    let Box1(b) = b1;

    let p1 = Point { x: 3, y: 4 };
    let Point { ref x, .. } = p1;

    let b2 = Some(Box::new(1));
    let b3 = &Box1("S".to_string());
    // let s1 = b3.0; // & 引用绑定是不能把字段的所有权转移出去的，除非 Copy
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
    let mut a = String::new();
    // a += 4;
    let b = &mut a;
    // let c = &mut a;  // 只允许转移一个可变引用
    // a.push_str("sd");
    b.push_str("sd");
    // c.push_str("sd");
    a.push_str("sds"); // 把 b 的可变引用解除了

    let mut a = 3;
    let a2 = &mut a;
    *a2 = 4;
    assert_eq!(4, a);

    let mut a = String::new();
    let a1 = &a;
    let a2 = &a;
    a1.len(); // 可以有多个只读引用，a1 并没有失效    
    {
        let a3 = &mut a; // a1 和 a2 不可用了，只能有一个可变引用
        // 【问1】不明白为什么要让 a1 a2 引用失效，难道是要让上下文更简洁，因为 a3 包含了 a1 a2 的所有功能，所以强制使用 a3
        // a1.len(); // err
        a3.push_str("ds");
    }
    // a2.len(); // 这也不行，&mut a 在上一步转移了可变性，这意味着此时的 a2 和上次 a2 的值不一样。
    // 【问2】至于这个性质，对 Rust 来说并没有明显的用处，而且感觉徒增负担。能详细解释一下吗。
    // 就算 a3 是另一个线程里面的，那也是传达了一个 a2 和上次的值不一样，该使用 a2 还是照样使用 a2
    let a2 = &a;
    a2.len();


}

struct A { a: String, b: String }

fn mut_struct() {
    let mut a = A { a: String::new(), b: String::new() };
    let a1 = &mut a;
    let a2 = a1; // a1 所有权转移了，a1 失效了
    // a1.a.len();
    let aa = &mut a2.a;
    aa.push_str("ds");
    a2.a.push_str(" ds ");// aa 失效了，只能有一个可变引用
    // aa.push_str(" ds "); // err
    
    let aa = &a2.a;
    let bb = &a2.b;
    bb.len();
    a2.a.len();
    bb.len();
    a2.b.push_str("sd"); // 一次可变的操作，影响到了 bb 这个绑定
    // bb.len(); // err
    aa.len();
}

fn ref1(mut obj1: A) {
    let a = String::new();
    let ref a1 = a;
    let mut a1 = a;
    a1.push_str("ds");

    let ref mut a2 = a1;
    a2.push_str("ds  ");
    // let ref ref a3 = a1; // err
    
    // let A { &a, b: @ bb : _ } = obj1; // a 和 b 解构语法都是错的
    let A { ref mut a, b: ref bb } = &mut obj1;
    a.push_str("ds");
    // bb.push_str("ds");
    bb.len();
    obj1.a.push_str("ds");
    obj1.a.len();

    let A { a, b } = &mut obj1;
    a.len();
}


enum Message {
    Quit,
    WriteString(String),
    Move { x: i32, y: i32 },
    ChangeColor(u8, u8, u8),
    T4(u8, u8, u8, u32),
}
fn pattern1() {
    let message = Message::Quit;
    match message {
        Message::Quit => println!("Quit"),
        Message::WriteString(write) => println!("{}", &write),
        Message::Move{ x: xx, y: 0 } => println!("move {} horizontally", xx),
        Message::Move{ .. } => println!("other move"),
        Message::T4(a, .., d) => println!("{}", a as u32 + d),
        Message::ChangeColor { 0: red, 1: green, 2: _ } => { // 这也可以
            println!("color change, red: {}, green: {}", red, green);
        }
    };

}

fn pattern2() {
    let r1 = 1..5;
    match &r1 {
        // &(2..3) => (),   // err 这个是无法匹配的
        std::ops::Range{ start: 1, end } if *end < 5 => (),
        _ => (),
    }

    match 3 {
        1..=32 => (), // 为什么 integer 类型可以匹配 Range 类型，特殊处理。。
        // 1..3 => (), // err 编译器不会认为是 Range 类型，肯定有特殊处理
        // ..3 => (), // 语法错误，这个也不是 Range 类型，语法上就不通过了，而不是类型错误
        _ => (),
    }
}

fn pattern_slice() {
    let a = [1_u8, 3, 4, 5];
    let res1 = match &a {
        [a1, _, a2, 5] => a1 + a2 + 1,
        [a1, .. , a2] => a1 + a2,
    };

    assert_eq!(6, res1);
}
