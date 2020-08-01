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
