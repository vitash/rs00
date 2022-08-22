#[test]
fn test() {
    struct_move::test2();
}
struct A<'a>(&'a str);

fn struct1() -> A<'static> {
    let s1 = "ds";
    let a1 = A(s1);
    a1
}

fn struct2() -> u8 {
    let s1 = String::new();
    let a1 = struct22(s1.as_str());

    fn struct22<'a>(s: &'a str) -> A<'a> {
        A(s)
    } // 这个没有返回值，这个是语句
    3_u8
}

fn fn_ref1(ref a: String) {
    if let Some(b) = &Some(1) {}
    if let Some(ref b) = &Some(1) {}
}
fn fn_ref2() {
    let a = "".to_string();
    fn_ref1(a); // fn_ref1 需要的参数还是 String, 只不过它的内部匹配进行解引用
                // let b = a.to_string(); // err
}

pub mod struct_move {
    #[derive(Debug)]
    struct A(u8);
    impl Drop for A {
        fn drop(&mut self) {
            println!("{:?}", self);
        }
    }
    fn test(a: A) -> A {
        // a.0 += 1;
        println!("{:p}", &a);
        a
    }
    pub fn test2() {
        let a = A(0);
        println!("{:p}", &a);

        let mut a = a;
        a.0 += 1;
        println!("{:p}", &a);

        let a2 = test(a);
        println!("{:p}", &a2);
    }
}

pub mod mut0 {
    fn fn1(s: &mut String) {}
    fn fn2() {
        let mut s = "".to_string();
        let s1 = &mut s;
        fn1(s1); // s1 没有失效
        s1.len();
        {
            let s2: &mut String = s1; // s2 的生命周期只是当前作用域？
        }
        s1.len(); // s1 没有失效
        {
            let s2 = s1; // s1 的生命周期直接给了 s2 ？
        }
        // s1.len(); // s1 失效了
    }
    #[derive(Debug)]
    struct NumRef<'a>(&'a i32);

    impl<'a> NumRef<'a> {
        // my struct is generic over 'a so that means I need to annotate
        // my self parameters with 'a too, right? (answer: no, not right)
        fn some_method(&'a mut self) {}
    }

    fn main() {
        let mut num_ref = NumRef(&5);
        num_ref.some_method(); // mutably borrows num_ref for the rest of its lifetime
                               // num_ref.some_method(); // compile error
                               // println!("{:?}", &num_ref); // also compile error
    }
}

mod m1 {
    struct Buffer {
        buffer: String,
    }
    struct Render {
        current_buffer: Buffer,
        next_buffer: Buffer,
    }
    //实现结构体 `Render` 的方法
    impl Render {
        //实现 update_buffer() 方法，
        //更新buffer，把 next 更新到 current 中，再更新 next
        fn update_buffer(&mut self, buf: String) {
            // let n = ;
            // self.current_buffer = self.next_buffer;
            // self.next_buffer = Buffer { buffer: buf };
        }
    }
}

mod nll {
    #[test]
    fn test14_2_example1() {
        let v1 = &mut vec![1, 2, 3];
        v1.push(v1.len());
    }

    #[test]
    fn test14_2_example2() {
        let v1 = &mut vec![0, 1, 2, 3];
        let mut p1 = &mut v1[0];
        println!("{p1}");
        v1.push(4); // p1 is dead
        p1 = &mut v1[2]; // p1 is live again
        println!("{p1}");
    }
}

#[test]
fn test_tup1() {
    let mut a = 1;
    let refa = &mut a;
    let t1 = &mut (refa, &2);
    let (x, y) = t1;
    // *t1.0 += 2;
    **x += 2;

    println!("{x:?}");
}

mod borrow {
    use std::borrow::Borrow;

    struct S1();
    impl std::borrow::ToOwned for S1 {
        type Owned = S2;

        fn to_owned(&self) -> Self::Owned {
            todo!()
        }
    }
    #[derive(Debug)]
    struct S2();
    impl std::borrow::Borrow<u32> for S2 {
        fn borrow(&self) -> &u32 {
            &32
        }
    }
    impl std::borrow::Borrow<S1> for S2 {
        fn borrow(&self) -> &S1 {
            todo!()
        }
    }
    impl AsRef<u8> for S2 {
        fn as_ref(&self) -> &u8 {
            &9
        }
    }

    // #[test]
    fn test_s2(s2: &S2) {
        let x: &S1 = s2.borrow();
        let y: &S1 = s2.borrow(); // 算是多个只读引用
        println!("{s2:?}");
        // let z = s2.borrow::<u32>();
    }
}

mod xp_bm {
    fn test_box<'a>(s: Box<&'static str>) {
        let local: Box<&'a str> = s;
    }

    fn test_arg<'a>(f: fn(&'a mut str)) {
        let local: fn(&'static mut str) = f;
        // 这个只是指针的内容可变，但是指针是不能更改指向的
        // 'static 是所有声明周期的底层类型
        // 顶层类型：所有类型的基类
        // 底层类型：它可以赋值给所有类型，是所有类型的子类
    }

    // 把 'static 当成底层类型，'static 是 'a 的子类型，返回值是协变的，书上说逆变是错的吧
    fn test_ret<'a>(f: fn() -> &'static str) {
        let local: fn() -> &'a str = f;
    }

    use std::cell::Cell;
    fn test_cell<'a>(s: Cell<&'a str>) {
        // Cell 特殊设计成参数不变的
        // let local: Cell<&'static str> = s;
    }

    // 裸指针是不可变的，书上应该是过时了
    fn test<'a, 'b: 'a>(s: *mut &'a str) {
        // let local: *mut &'b str = s; // 如果可以，这里是逆变的
    }

}
