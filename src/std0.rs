use std::mem::size_of;
use std::mem::transmute;

#[test]
fn test() {
    pointer1()
}
fn float_max() {
    let max = 0.3_f32.max(f32::NAN);
    assert_eq!(0.3, max);
    assert_eq!(false, f32::NAN < 0.3_f32);
}
fn sizeof() {
    assert_eq!(8, size_of::<usize>());
    assert_eq!(16, size_of::<Option<usize>>());
    assert_eq!(16, size_of::<Option<Option<usize>>>());

    assert_eq!(2, size_of::<Option<u8>>());
    assert_eq!(16 + 1 + 7, size_of::<Option<u128>>());
    assert_eq!(
        2,
        size_of::<Option<Option<Option<Option<Option<Option<Option<Option<Option<Option<u8>>>>>>>>>>>()
    ); // 这都10个 Option 了还是16位，2个字节. 枚举类型的优化没弄懂
    assert_eq!(8, size_of::<Option<&u8>>()); // 一个正确的指针一定不为 0
    assert_eq!(8, size_of::<Option<Box<u8>>>()); // Rust 有所优化，0 表示为空指针

    assert_eq!(8, size_of::<&u8>());
    assert_eq!(8, size_of::<Box<u8>>());
    assert_eq!(8, size_of::<&()>());
    assert_eq!(0, size_of::<[(); 2]>());
    assert_eq!(2, size_of::<(u8, u8)>());
    assert_eq!(3 * 4, size_of::<[(u8, u8, u8); 4]>());
    assert_eq!(3 * 3, size_of::<[(u8, u8, u8); 3]>()); // == 9, 对齐的事自然会有底层处理

    assert_eq!(16, size_of::<&[u8]>()); // 16, 索引开始和结束
    assert_eq!(24, size_of::<Vec<u8>>()); // RawVec，len
}

fn pointer1() {
    println!("{:?}", size_of::<&[i32; 3]>());
    println!("{:?}", size_of::<&[i32]>());

    let v: [i32; 5] = [1, 2, 3, 4, 5];
    let p: &[i32] = &v[2..4];
    unsafe {
        let (ptr, len): (usize, u8) = transmute(p);
        println!("{} {}", ptr, len);

        let ptr = ptr as *const i32;
        for i in 0..len {
            println!("{}", *ptr.offset(i as isize));
        }
    }

    let p = &();
    println!("&() \t {:p}", p);
    let p = Box::new(());
    println!("box(()) \t {:p}", p);

    let x = Box::new(());
    let y = None::<Box<()>>;
    let z = Some(Box::new(()));
    let z2 = Some(0_u32);
    let z3 = Some(0_u16);
    let z4 = Some(1_u32);
    let z5 = Some(23_u32);
    unsafe {
        let value1: usize = transmute(x);
        let value2: usize = transmute(y);
        let value3: usize = transmute(z);
        let (z2, z3, z4, z5): (usize, u32, usize, usize) = (transmute(z2), transmute(z3), transmute(z4), transmute(z4));
        println!("{} {} {}, {} {} {} {}, ", value1, value2, value3, z2, z3, z4, z4);
    }
}

use std::fmt::Debug;
fn call_sized<T: Debug + ?Sized>(p: &T) {
    println!("{:?}", p);
}

fn main() {
    let x: &[i32] = &[1, 2, 3, 4];
    call_sized(x); // 如果不加 T: ?Sized 编译错误，泛型默认有一个 T: Sized 的约束
}
