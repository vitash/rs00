#[allow(unused_macros)]
mod proc1;
mod exemple1;

fn std_macros() {
    println!("{}", 2);
    stringify!(22);
}

#[test]
fn test() {
    path1()
}

macro_rules! each_tt {
    () => ();
    ( $_tt:tt $($rest:tt)* ) => ( each_tt!($($rest)*));
}

fn trace_macros2() {
    trace_macro();
    each_tt!(aa bb); // 这里不显示，居然是按行号来的，像预编译指令
}

fn trace_macro() {
    trace_macros!(false);
    each_tt!(aa bb);
}

trace_macros!(false); //如果没有，会贯穿到下面的代码

fn path1() {
    macro_rules! call_foo {
        () => {
            $crate::macros::mr_keyword()
        };
        (p) => {
            panic!("panic err") // 会当做代码展开，而不是产生编译错误信息
        };
        ($x:ty) => {
            compile_error!("err x {}", $x);
        };
    }
    call_foo!(p);
    // call_foo!(u8);
}

fn macro_rules() {} // ok
fn mr_keyword() {
    macro_rules! macro_rules (
        ($x:expr) => ( Some($x) )
    );
    println!("{:?}", macro_rules!(3 + 3));
}

fn m_tt() {
    macro_rules! foo {
        ($l:tt) => {
            bar!($l);
        };
    }

    macro_rules! bar {
        (3) => {};
    }

    foo!(3);
    // foo!(4);  //err

    macro_rules! text1 {
        (abc) => {};
    }
    let abc = 3;
    text1!(abc);
}

fn fn_vec2() {
    macro_rules! vec2 {
        // ( $( $x:expr ) / * ) => {
        ( $( $x:expr )* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }
    let v1 = vec2!(3 4 5 6);
    println!("{:?}", v1);

    // macro_rules! tuple2 (
    //     ($a:expr $b:expr) => (($a, $b))
    // );
    // let t1 = tuple2!(3 4);
    // println!("{:?}", t1);

    macro_rules! tuple (
        ($( $a:expr )+) => (($( $a, )+))
    );
    let t1 = tuple!(3 + 2 4 5); //容易有歧义
    println!("{:?}", t1);
}

fn fn1() {
    macro_rules! pat {
        ($i:ident) => {
            Some($i)
        };
    }
    let x = 3;
    assert_eq!(pat!(x), Some(3));
}

fn test2() {
    let x = 3;
    match x {
        0..=4 => println!("0..4"),
        2..=6 => println!("2..6"),
        _ => (),
    }
}
