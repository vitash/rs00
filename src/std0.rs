use std::mem::size_of;
#[test]
fn test() {
    sizeof()
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
