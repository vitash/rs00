use std::fmt;
use std::ops::Add;

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
    test_deref()
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
    double_first(vec);
    double_first(&["3", "33"]);
    // double_first(&vec.iter().skip(3));
}