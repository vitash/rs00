

fn fn1() {
    let mut s1 = "111-".to_string();
    fn2(&mut s1);
    println!("{}", &s1);
    let a: &str = "ss";
}

fn fn2(s1: &mut String) {
    let mut s2 = "dsd".to_string();
    s1.push_str(s2.as_str());
    s2.push_str("22222");
    println!("s2 = {}", &s2);
}

#[test]
fn test1() {
    // main_test();
}

