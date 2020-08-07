trait Trait1 {
    type O1; // 关联类型
    fn fn1() -> Self::O1;
}

impl Trait1 for u8 {
    type O1 = impl std::cmp::Ord; // #![feature(type_alias_impl_trait)]
    fn fn1() -> Self::O1 {
        3 // type O1 = impl std::cmp::Ord; // 这个应该是没用的，因为只能实现 Trait1 一次，返回值已经确定了
    }
}

// impl Trait1 for u8
// where // 这个约束是不允许约束的
//     <Self as Trait1>::O1: std::cmp::PartialEq,
// {
//     type O1 = u32;
//     fn fn1() -> Self::O1 {
//         3
//     }
// }

trait Tr2 {
    fn fn1();
}

impl<T> Tr2 for T
where
    T: Trait1,
    <T as Trait1>::O1: Tr2, // 这里也是可以约束得到，关联类型和泛型的区别在哪？
{
    fn fn1() {}
}

// 关联类型只能用于声明 trait，泛型可以用于声明 struct \ trait
// where 的约束只能是 trait，但是关联类型可以是 struct \ trait

mod t1 {
    trait TeHx {
        fn data(&self) -> String;
    }
    impl<T: std::fmt::Debug> TeHx for T {
        fn data(&self) -> String {
            format!("{:?}", self)
        }
    }
    // impl TeHx for u8 {} // 类型错误不可实现

    trait TeHx2: TeHx {}
    impl TeHx2 for u8 {
        // err: method `data` is not a member of trait `TeHx2`
        // fn data() -> String {
        //     format!("{:?}, u8", self)
        // }
    }
}

mod t2 {
    trait TeHx<T: TeHx<T> + Sized> {
        fn data(&self) -> String;
    }
    // trait TeHx3: TeHx<TeHx3> {}  // 直接就类型循环引用了

    // impl<T: std::fmt::Debug> TeHx<TeHx2> for T {
    //     fn data(&self) -> String {
    //
    //     }
    // }
    // trait TeHx2: TeHx<TeHx3> + std::fmt::Debug {
    //     fn spec_data() -> String {
    //         format!("{:?}", self)
    //     }
    // }
    // impl TeHx2 for u8 {
    //     fn spec_data() -> String {
    //         format!("{:?}, u8", self)
    //     }
    // }
}
mod t3 {
    trait TeHx {
        // fn data(&self) -> String;
        fn spec_data(&self) -> Option<String> {
            None
        }
    }
    impl TeHx for u8 {
        fn spec_data(&self) -> Option<String> {
            Some(format!("u8: {}", self))
        }
    }
    trait TeHx2: TeHx {
        fn get_data(&self) -> String;
    }
    impl<T: std::fmt::Debug + TeHx> TeHx2 for T {
        fn get_data(&self) -> String {
            match self.spec_data() {
                None => format!("{:?}", &self),
                Some(data) => data,
            }
        }
    }

    #[test]
    fn test() {
        let a = 9_u8;
        assert_eq!("u8", a.get_data().as_str());
        let a2 = 9_u32;
        // a2.get_data(); 没有实现啊，靠
    }
}


use std::marker::PhantomData;