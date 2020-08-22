use std::borrow::Cow;

#[derive(Debug)]
struct Point {
    x: u8,
    y: u8,
}

#[test]
pub fn test() {
    ref_cell1();
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
        Message::Move { x: xx, y: 0 } => println!("move {} horizontally", xx),
        Message::Move { .. } => println!("other move"),
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
        std::ops::Range { start: 1, end } if *end < 5 => (),
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
        [a1, .., a2] => a1 + a2,
    };

    assert_eq!(6, res1);
}

fn test_life(a1: &u8) {
    let b = &3;
    let r = life1(a1, b);
}

fn life1<'a, 'b: 'a>(a: &'a u8, b: &'b u8) -> &'a u8 {
    a.max(b)
}

fn closure1() {
    let mut a = 3;
    // let f = &a;
    let f1 = |x: i32| x + a;
    a = 4;
    let b = &mut a;
    *b = 10;
}

// fn cow1() {
//     let mut cow: Cow<[_]> = Cow::Owned(vec![1, 2, 3]);
// }

// fn get_str(s: &String) -> &str {  // 这个签名实际上是下面的隐式
// fn get_str<'a>(s: &'a String) -> &'a str { 
// &'static str -> &'a str, 这个当然是可以的，但是编译器没有把返回值声明为 'static
fn get_str(s: &String) -> &'static str {
    println!("call fn {}", s);
    "hello world"
}

fn lifetime2() {
  let c = String::from("haha");
  let x: &'static str = get_str(&c);
  println!("{}", x);
}


use std::mem::replace;

pub struct IterMut<'a, T: 'a> { data: &'a mut[T] }

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        let d = replace(&mut self.data, &mut []);
        if d.is_empty() { return None; }

        let (l, r) = d.split_at_mut(1);
        self.data = r;
        l.get_mut(0)
    }
}
fn test_itermut(v1: &mut [u8]) -> Option<&mut u8> {
    let mut it1 = IterMut { data : v1 };
    it1.next()
}
fn test_itermut2() {
    let mut v1 = vec![1, 2, 3];
    let a = test_itermut(&mut v1);
    assert_eq!(Some(&mut 1), a);
}

use std::mem::transmute;

#[derive(Debug)]
struct ST1 {
    member: i32,
}

fn test_st1<'a>(arg: &ST1) -> &i32 // 返回值的生命周期认为是 'a
{
    println!("{:?}", arg);
    let local = ST1 { member : 12 };
    unsafe {
        transmute(&local.member as *const i32)
    } // local 已经被释放了
}

fn test_unsafe1() {
    let t = ST1 { member : 10 };
    let x = test_st1(&t);
    println!("x: {:?}", x);
    assert_eq!(&0, x);
}


use std::fmt::Debug;

#[derive(Debug)]
enum EA2 {
    A(Vec<u8>),
    B(u32),
}

fn mut_enum() {
    let a = &mut EA2::A(vec![]);
    if let EA2::A(v) = &a {
        // *a = EA2::B(32); // error
        v.len(); // 如果取消上面的注释，这里将是不安全的代码, a 地址上的内容已经改变了
        *a = EA2::B(32);
        println!("{:?}", a);
    }
    
    // 共享不可变，可变不共享
    let mut a1 = 3_u8;
    let a2 = &a1; // 只读共享
    let a3 = &a1;
    let r1 = a2 + a3;
    a1 = 4; // 上面的所有共享将会失效, 
    // let r2 = a2 + a3; // error
    // 这对于单线程，类型不可变的情况都是安全的，为什么要禁止，不允许通过编译（注：下面的 V1::test 里面显露出了原因）
    a1.to_string();
}

fn mut4() {
    let mut arr = vec!["ABC", "DEF", "GHI"];
    let mut it1 = arr.iter();
    let x = it1.next();
    arr.clear(); // arr 再一次改变，前面的引用都失效了
    // let x = it1.next(); // 编译期就报错了，好强
    // 但是他怎么知道 Iter 和 Vec 有关联
}

struct V1;
struct It1<'a>(&'a u8);
impl V1 {
    fn iter(&self) -> It1 { It1(&3) }
    fn clear(&mut self) { }
    fn test() {
        let mut v1 = V1;
        let mut it1 = v1.iter();
        it1.0 = &22;
        v1.clear(); // 是生命周期标记，如果没有生命周期约束，这两个类型是无关的，是生命周期无关的
        // it1.0 = &33; // error
    }
}

impl std::ops::Drop for V1 {
    fn drop(&mut self) {
        println!("V1 Drop");
    }
}

fn mut5(s1: &mut Vec<V1>) -> &mut Vec<V1> {
    *s1 = vec![V1];
    // 可变性和所有权是相关的，比如这里，s1 之前的已经被 drop 掉了，返回的是一个新的
    // 感觉可变性的权限比所有权还有大，它同样拥有了所有权的功能
    // let s2 = *s1; // 把所有权转移给别人，不允许

    println!("mut5 end; {:p}", &*s1);
    s1
}

fn mut52() {
    let mut v1 = vec![V1];
    let a1 = &v1[0]; // 这是取的是堆的地址
    // a1 指向的那个内存仅在 v1 不可变的范围内有效，一旦v1改变了，无法保证这里的指向还是存活的
    // 【问1】如果是堆上的内存，那么这个指向就失效了，如果是栈上的，那这也仅仅只是被替换而已，指向还是有效的
    // 不，如果是栈上，这整个 Vec<V1> 的这片内存也被替换了，a1 指向还是失效了

    let a2 = &v1; // 这里取的是栈的地址
    println!("mut5 start; {:p}; {:p}", &v1, a2);
    mut5(&mut v1); // V1 在里面已经被释放了，之前的所有引用都应该失效
    println!("mut55 end; {:p}", &v1); // v1 只是被替换了，没有发生指向失效，所以 a2 应该是可用的
}

// 可变性：替换和指向
// 替换是指位置不变，内容被替换了
// 指向改变，指向位置已经变了，内容也不是原来的内容了

fn mut6(v1: &mut Vec<V1>, v2: &mut Vec<V1>) {
    // *v1 = *v2; // 不可以单独提取所有权(*v2)，却可以把所有权转移(*v)，甚至在这里进行释放，
}
fn mut61(v1: &mut [V1;3], v2: &mut [V1;3]) {
    *v1 = [V1, V1, V1]; // v1 的内容被替换了，指向没有发生改变，这里没有发生在堆上
}
fn mut62() {
    //为什么一些可变引用是安全的，有些是危险的，
    let mut a1 = 3;
    let a2 = &mut a1;
    a1 = 33;
    // println!("{}", a2); // 我认为 a2 应该还是安全的，还是可以使用的，
    println!("{}", &a1);

    let mut v1 = vec![1_u8, 2, 3];
    let it1 = &mut v1.iter();
    v1.clear();
    // it1.next(); // 这里是不安全，应该被禁止，如何区分这两种情况，这里为什么是不安全的
    // 迭代器里面可能记录了一些旧的数据记录，比如开始位置，结束位置。
    // 进行迭代就会出错，虽然 iter.next() 返回的是一个 Option，因为他每次都去访问 vec，如果是一个 Range，那么这个 Range 就是失效的
    // 归因于此，是类型变了，导致内容变了，为什么 v1.clear 之后就是另一个类型了，确实是这样，类型变成了[u8;0]
    // v1 是一个数组，但是长度是变化的，内存里的长度是会变的，不像栈上的数组是固定的，所以 [u8;2] 和 [u8;3]，不应该作为同一个类型
    // 对于枚举类型，它也是变化的，内存的长度也会改变，类型的可变导致这些可变操作是不安全的元凶，而替换是可变的安全操作，只是内容替换了
    // 要限定就应该限定类型可变性，如果一个操作导致类型会发生改变，就应该使得前面的引用失效，如果只是普通的内容替换，这个可变操作是允许的
}

use std::cell::Cell;

fn cell1() {
    let data : Cell<i32> = Cell::new(100);
    let p = &data;
    data.set(10);
    println!("{}", p.get());

    p.set(20);
    println!("{:?}", data);

    let a2 = Cell::new(EA2::B(3));
    let p = &a2;
    a2.set(EA2::A(vec![]));
    // println!("{:?}", p.get());
}

struct Cell1<T>(T);
impl<T> Cell1<T> {
    fn set(&self, val: T) {
        // &self 这签名怎么可以改内部成员
        unsafe {
            let p = &self.0 as *const T as *mut T;
            *p = val;
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

fn ref_cell1() {
    let shared_vec: Rc<RefCell<_>> = Rc::new(RefCell::new(vec![1, 2, 3]));
    let shared1 = shared_vec.clone();
    let shared2 = shared1.clone();

    let mut mut_borrow1 = shared1.borrow_mut();
    mut_borrow1.push(4);
    // drop(mut_borrow1);

    let mut mut_borrow2 = shared2.borrow_mut();
    mut_borrow2.push(5);

    println!("{:?}", shared_vec);
}

mod node1 {
    struct Node {
        next : Option<Box<Node>>
    }

    fn ref_cell2() {
        let mut node1 = Box::new (Node { next : None });
        let node2 = Box::new (Node { next : None });

        node1.next = Some(node2);
        match &mut node1.next {
            // Some(node) => node.next = Some(node1),
            _ => ()
        }
    }
}

mod node2 {
    use std::rc::Rc;
    use std::cell::RefCell;
    struct Node {
        next : Option<Rc<RefCell<Node>>>
    }

    fn ref_cell2() {
        let node1 = Rc::new(RefCell::new(Node { next : None }));
        let node2 = Node { next : None };

        node1.borrow_mut().next = Some(Rc::new(RefCell::new(node2)));
        match &node1.clone().borrow().next {
            Some(node) => node.clone().borrow_mut().next = Some(node1),
            _ => ()
        }
    }
    
    fn ref_cell3() {
        let node1 = Rc::new(RefCell::new(Node { next : None }));
        let node2 = Rc::new(RefCell::new(Node { next : None }));

        node1.borrow_mut().next = Some(node2.clone());
        node2.borrow_mut().next = Some(node1);
    }

    fn ref_cell4() {
        let node1 = Rc::new(RefCell::new(Node { next : None }));
        let node2 = Rc::new(RefCell::new(Node { next : None }));
        let node3 = Rc::new(RefCell::new(Node { next : None }));

        node1.borrow_mut().next = Some(node2.clone());
        node2.borrow_mut().next = Some(node3.clone());
        node3.borrow_mut().next = Some(node1.clone());
    }
}

fn cell2() {
    let mut data = [100_i32];
    let p = &data;
    data = [10];
    // println!("{:?}", unsafe { *p }); // p 应该还是有效的地址，我想看一下值，但是我写不出来。。。
    println!("{:?}", data);
}