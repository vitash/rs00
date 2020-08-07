#[test]
fn test() {
    struct1();
}
struct A<'a>(&'a str);

fn struct1() -> A<'static> {
    let s1 = "ds";
    let a1 = A(s1);
    a1
}

fn struct2() -> u8 {
    let s1 = String::new();
    let a1 = struct22(s1.as_str());

    fn struct22<'a>(s: &'a str) -> A<'a> {
        A(s)
    } // 这个没有返回值，这个是语句
    3_u8
}
