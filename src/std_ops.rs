use std::fmt;
use std::ops::Add;
use std::marker;

// type NeverErr<T> = Result<T, !>;
struct A(u8, u8);

impl Add<u8> for A {
    type Output = u8;
    fn add(self, rhs: u8) -> Self::Output {
        self.0 + rhs
    }
}

impl std::ops::Index<usize> for A {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            _ => panic!("out of range"),
        }
    }
}

#[test]
fn test() {
    test_into_iter()
}

struct AA {
    aa: u32,
}
impl AA {
    #[inline]
    fn get_aa(&self) -> u32 {
        self.aa
    }
}
struct AA2 {
    a: AA,
    b: u32,
}
impl std::ops::Deref for AA2 {
    type Target = AA;
    fn deref(&self) -> &Self::Target {
        &self.a
    }
}

fn test_deref() {
    let a2 = AA2 { a: AA { aa: 33 }, b: 30 };
    assert_eq!(33, a2.get_aa())
}
fn test_deref2() {
    use std::rc::Rc;
    let s = Rc::new(Rc::new(String::from("hello")));
    let s1 = s.clone();
    let ps1 = (*s).clone();
    let pps1 = (**s).clone();

    let s = String::new();
    let s2 = &s;
    // 配平 & 的艺术
    match (&*s, s2.as_str(), &**s2) {
        ("", "", "") => {}
        _ => {}
    }
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
    else_return!(c); // 只是针对 `Option` 类型，`Result` 不管它有 `?`, `try!` 做处理

    let c2 = c; // rust-analyzer，类型推断错的，实际是 u8 类型
    assert_eq!(3_u8, c2); // u8 类型，编译正确
}
struct Opt1(u8);
impl std::ops::Try for Opt1 {
    type Ok = Self;
    type Error = ();
    fn from_error(v: Self::Error) -> Self {
        Opt1(0)
    }
    fn from_ok(v: Self::Ok) -> Self {
        v
    }
    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        Ok(self)
    }
}
fn option2(a: Option<Opt1>) -> Option<()> {
    let a = a?;
    None
}

fn result1() {
    let r0 = Ok::<_, u8>(String::new());
    let r1 = &r0;
    match *r1 {
        // 这也可以，引用类型获取所有权，然而并没有发生所有权转移
        Ok(ref s) => (),
        Err(e) => (), // 这个是因为拷贝了
    };

    // let r2 = *r1; // 编译错误，所有权发生了转移
    // match r2 {
    //     Ok(ref s) => (),
    //     Err(e)=> (),
    // };
}

fn double_first(vec: &[&str]) -> Option<Result<i32, std::num::ParseIntError>> {
    vec.first().map(|first| first.parse().map(|n: i32| 2 * n))
}
fn test_double_first(vec: &Vec<&str>) {
    // double_first3(&vec);
    // double_first3(&["3", "33"]); // 无法实现，参考 use std::process::Command::args
    // double_first3(&vec.iter().skip(3));
}
fn double_first2<'a>(mut vec: impl Iterator<Item = &'a &'a str>) -> Option<Result<i32, std::num::ParseIntError>> {
    vec.next().map(|first| first.parse().map(|n: i32| 2 * n))
}
// fn double_first3<T: AsRef<str>>(vec: &impl IntoIterator<Item = &'a T>) -> Option<Result<i32, std::num::ParseIntError>> {
//     vec.into_iter().next().map(|first| first.as_ref().parse().map(|n: i32| 2 * n))
// }

fn res_iter() {
    let a = Ok::<_, u32>(3);
    let it = &mut a.iter();
    assert_eq!(Some(&3), it.next());
    assert_eq!(None, it.next());

    let it = a.iter();
    let it2 = it.into_iter();
    let take = it2.take(3);
}

fn res_collect1() {
    let strings = vec!["1", "tofu", "93", "18"];
    let numbers = strings.iter().map(|s| s.parse::<i32>()).collect::<Result<Vec<i32>, _>>();
    println!("Results: {:?}", numbers);
}

fn test_into_iter() {
    let strings = vec!["1", "tofu", "93", "18"];
    let iter = &mut strings.iter();
    let new_iter = iter.into_iter(); // &mut 怎么还能调用，因为实现了...哈哈
    let a = iter.next();
    assert_eq!(Some(&"tofu"), iter.next());
    // new_iter 也是 &mut 类型，而且生命周期也是一样的
    // 卧槽，发现了什么东西，可以同时拥有两个 &mut 绑定，再看下一行
    // assert_eq!(Some(&"93"), new_iter.next());  // 想多了， &mut 也是有所有权的，
}
