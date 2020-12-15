trait  T1 {
}
trait H1<A> {
    fn fn_h1(a: A);
}

// err: the type parameter `A` is not constrained by the impl trait, self type, or predicates
// impl<T: H1<A>, A> T1 for T {
// }

struct S1;
