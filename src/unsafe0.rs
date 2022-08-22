use std::{mem, ptr};

#[test]
fn write_u64() {
    let mut y: u32 = 1;
    let x = 1_u32;
    let raw_mut = &mut y as *mut u32 as *mut i32 as *mut u64; // 这是安全的
    unsafe {
        *raw_mut = 0xFFFF_0000_F000_FFFF; // 这是不安全的，必须在 unsafe 块中才能通过编译
    }
    println!("{:X} {:X}", x, y);
}

#[test]
fn null_ptr() {
    let p1 = std::ptr::null::<u32>();
    println!("{p1:p}");
    let p2 = 0 as *const u32;
    println!("{p2:p}");
    dbg!(p2.is_null());
}

fn swap<T>(x: &mut T, y: &mut T) {
    unsafe {
        let mut t: T = mem::uninitialized();
        ptr::copy_nonoverlapping(x, &mut t, 1);
        ptr::copy_nonoverlapping(y, x, 1);
        ptr::copy_nonoverlapping(&t, y, 1);
        mem::forget(t);
    }
}

#[test]
fn test_swap() {
    let (a, b) = &mut (3, 4);
    swap(a, b);
    println!("{a}, {b}");
}

#[test]
fn alias_struct() {
    struct Foo {
        a: i32,
        b: i32,
        c: i32,
    }

    let mut x = Foo { a: 0, b: 0, c: 0 };
    let pa = &mut x.a;
    let pb = &mut x.b;
    *pb += 1;
    *pa += 10; // 存在两个可变引用，还是正常编译
    println!("{} {}", pa, pb);

    let mut x = [1_i32, 2, 3];
    let pa = &mut x[0];
    // let pb = &mut x[1];
    // 这个却不能借用了两个可变引用
}

mod drop_same_time {

    struct T {
        dropped: bool,
    }
    impl T {
        fn new() -> Self {
            T { dropped: false }
        }
    }
    impl Drop for T {
        fn drop(&mut self) {
            println!("droppen T: {}", self.dropped);
            self.dropped = true;
        }
    }

    struct R<'a> {
        inner: Option<&'a T>,
    }
    impl<'a> R<'a> {
        fn new() -> Self {
            R { inner: None }
        }
        fn set_ref(&mut self, ptr: &'a T) {
            self.inner = Some(ptr);
        }
    }
    impl<'a> Drop for R<'a> {
        fn drop(&mut self) {
            if let Some(inner) = self.inner {
                println!("droppen R when T is {}", inner.dropped);
            }
        }
    }

    #[test]
    fn main() {
        {
            let (a, mut b): (T, R) = (T::new(), R::new());
            b.set_ref(&a);
        }
        {
            // let (mut a, b): (R, T) = (R::new(), T::new());
            // a.set_ref(&b);
            // b 先释放，a 后释放，这里是错的，a 的 drop 函数引用了 b
        }
    }
}
