
#[test]
fn test1() {
    run_fn1();
}

macro1::make_answer!();

fn t1_make_answer() {
    assert_eq!(42, answer());
}

fn local_fn1() {
    println!("local_fn1");
}

fn run_fn1() {
    macro1::run_fn1!();
}