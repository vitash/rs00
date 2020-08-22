fn max<T: std::cmp::PartialOrd<T>>(s: &[T]) -> &T {
    let mut m = &s[0];
    for x in s.iter() {
        if x > m {
            m = x
        }
    }
    m
}

fn type1() {
    enum E1<T> {
        O1(T),
        Err,
    };
    use E1::O1;

    let r1 = Ok::<u8, u32>(3);
    let e1 = O1::<u8>(3); // 这个泛型写法也是允许的，
}

#[test]
fn test_max() {
    let v1 = vec![1, 2, 3, -4];
    let m1 = max(&v1);
    let a = if m1 > &10 { 323 + 65 + 65 + 456 } else { 323 + 65 + 565 + 4665 + 456 };
    assert_eq!(m1, &3);
}

// fn lifetime1<T, 'a>() {} // err

struct LifeTime<'a>(&'a str);
struct LifeTime2<'a, T>(&'a str, T);
// a1 生命周期声明可以省略, 泛型参数不可以省略
fn lifetime2(a1: LifeTime2<'_, u32>, a2: LifeTime2<u32>) {
    let a: LifeTime = LifeTime(a1.0); 
    /* 所以为什么生命周期参数必须要放在类型参数的前面 */
}

