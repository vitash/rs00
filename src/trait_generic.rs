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

trait TeHx {}
impl<T: std::fmt::Debug> TeHx for T {}
// impl TeHx for u8 {} // 类型错误不可实现
