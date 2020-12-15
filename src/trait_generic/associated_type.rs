trait Fn1<'a> {
    type Args;
    fn fn_aa(a: Self::Args);
}

struct S1;

impl<'a> Fn1<'a> for S1 {
    type Args = (&'a String, u8);

    fn fn_aa(a: Self::Args) {
        println!("{:?}", &a);
    }
}

fn test(s1: &String) {
    S1::fn_aa((s1, 3))
}

#[test]
fn test_run() {
    test(&"sss".to_string());
}