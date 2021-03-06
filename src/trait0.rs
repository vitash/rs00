// use std::fmt::Debug;

trait Trait1 {
    // fn-name cjuu1 :@T cjuu2 :T  cjuu3: @U
    // { k: T }
    // : ;

    fn def2(&self)
    where
        Self: std::fmt::Debug,
    {
        println!("dsd: {:?}", self);
    }
    fn def1() {
        println!("dsd: {:?}", 1);
    }
    fn fn2() -> u8;
}

struct A1(u8);

impl Trait1 for A1 {
    fn fn2() -> u8 {
        3
    }
}

trait Tra1 {
    type Out1;
    fn tra1_fn1() -> Self::Out1;
}

fn fn3<T: Tra1<Out1 = u8>>(a: impl Tra1<Out1 = u32>, b: T, c: impl Trait1) -> u8 {
    // let o1 = <a as Tra1>::tra1_fn1(); // 想要调用 a 对应类型的 tra1_fn1 函数，（运行时行为？）
    let o2 = T::tra1_fn1();
    o2
}

trait Animal {
    fn baby_name() -> String; // 关联函数（静态函数？），可以有各自的实现，于传统的 interface 有所区别
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

#[test]
fn test_dog1() {
    assert_eq!(Dog::baby_name(), <Dog as Animal>::baby_name())
}

mod m1 {
    trait TA {
        fn fn_a() -> u8;
    }

    trait TB: TA {
        // 想为 TA 的 fn_a 做默认实现，这是 Rust 无法办到的
        fn fn_a() -> u8 {
            3
        }
        fn fn_b();
    }

    struct S1;
    //// 必须分开实现 trait
    // impl TA for S1 {}
    // impl TB for S1 {
    //     fn fn_b() {}
    // }

    // 本末倒置，某结构体实现 TB 必先实现 TA，用泛型特化做如此的实现无异于放屁
    impl<T: TB> TA for T {
        fn fn_a() -> u8 {
            3
        }
    }
}
