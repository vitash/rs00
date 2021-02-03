#[derive(Debug, Clone)]
struct Point(i8, i8);
pub fn test() {
    let p1 = Point(3, 33);
    // fn1(p1.clone());
    closure1(p1.clone(), p1.clone());
    closure2(p1.clone());
}

fn fn1(p1: Point) {
    let f1 = |a| p1.0 + a; // 虽然 p1 是 Point 类型，但是没有获取所有权，应该是捕获为 &Point
    println!("{:?}", p1);
    println!("{:?}", f1(f1(3)));
    // f1
}

fn fn2(f: impl Fn(i8) -> i8, a: i8) -> impl Fn(i8) -> i8 {
    move |x| a + f(x) // 必须添加 move，不懂为什么要 move f
}

fn closure1(mut p1: Point, mut p2: Point) {
    let mut add = || p1.0 += 4;
    // let mut add2 = || p1.0 += 10; // 不能捕获
    // add = || p2.0 += 3; // 也不能捕获
    add();
    // add();
    p1.0 += 10; // 可以同时存在两个 mut 引用 ?
    p2.1 += 20;
    println!("{:?}", p1);
}

fn closure2(p1: Point) {
    let f1 = || {
        println!("{:?}", p1);
        fn1(p1); // 这里 fn1 要求捕获的是 Point，所以这个闭包只能调用一次
                 // println!("{:?}", p1); // 错误, p1 value move
    };
    f1();
    // f1(); // 错误，只能调用一次

    let mut p2 = Point(3, 3);

    let mut f2 = move || {
        p2.0 *= 10;
        println!("{:?}", p2);
    };
    // println!("{:?}", p2); // 错误，所有权转移了
    f2(); // p2 所有权属于 f2
    f2(); // 多次调用
}

fn closure3(p1: Point) {
    // let
}

struct S1(String); // 这个也可以做函数指针
enum E1<T> {
    V1,
    V2(),
    Val(T),
    Other(u8, u8),
}
fn fn6() {
    fn fn61<T>(f1: fn() -> T) -> T {
        return f1();
    }
    let a = fn61(E1::V2::<u8>);
    // let a = fn61(E1::<u8>::V1);
    let a = E1::Other::<u8>;
}

fn fn5() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings = list_of_numbers
        .iter()
        .map(ToString::to_string) // <i32 as ToString>::to_string
        .map(S1)
        .map(E1::Val)
        .collect::<Vec<_>>();
}

trait Overload1 {
    fn func_aaa(&self, a: u32) {}
}

trait Overload2 {
    fn func_aaa(&self, a: &str) {}
}

impl Overload1 for u8 {}
impl Overload2 for u8 {}
fn overload(a: u8, xu32: u32, xstr: &str) {
    // a.func_aaa(xu32); // 错误
    // a.func_aaa(xstr); // 错误
}

fn as_str<T: Into<String>>(s1: T) {
    let s = s1.into();
    println!("{:?}", &s);
}

fn as_str2(s1: impl Into<String>) {
    let mut s = s1.into();
    s += " sss";
    println!("{:?}", &s);
}

fn test_as_str() {
    as_str2("ds");
    as_str2(String::new());
}

fn into_s1(s1: impl Into<S1>) {
    let s = s1.into();
    println!("{:?}", &s.0);
}
fn test_into_s1() {
    into_s1(S1(String::new())); // 他没有实现，但是还是能调用，是编译器实现了还是，编译过后优化掉了
}

pub enum PatternA {
    X(Vec<u8>, u8),
}

pub fn pattern_fn1<'a>(PatternA::X(a, b): &'a mut PatternA, v: Vec<u8>) {
    // let a2 = &a;
    // *a2 = &mut v;
    *a = v;
    a.push(3);
}

#[test]
fn move_num() {
    let mut num = 5;
    {
        // 把 num(5) 所有权给 move 到了 add_num 中，
        // 使用其成为闭包中的局部变量。
        let mut add_num = move |x: i32| num += x;
        add_num(5);
        println!("num(move)={}", num); //输出10

        // 此段代码抄自陈皓的博客，上面的输出并非是 10， 而是 5
        // https://coolshell.cn/articles/20845.html/comment-page-1#%E5%87%BD%E6%95%B0%E9%97%AD%E5%8C%85
    }
}
