use std::cell::RefCell;
#[test]
fn example_15_2_1() {
    let shared_vec: RefCell<Vec<isize>> = RefCell::new(vec![1, 2, 3]);
    let shared1 = &shared_vec;
    let shared2 = &shared1;
    let p1 = shared1.borrow();
    let p2 = &p1[0];
    // drop(p1);
    shared2.borrow_mut().push(4); // 运行时这里就出错了，值在 p1 那里，p1 要解除引用，p1 只是只读引用
    println!("{}", p2);
}

struct CellV1<T> {
    value: T,
}
impl<T> CellV1<T> {
    // fn set(&self, v: T) {
    //     self.value = v;
    // }
}

impl<T: Copy> CellV1<T> {
    fn new(v: T) -> Self {
        Self { value: v }
    }
    fn get(&self) -> T {
        self.value
    }
}

struct CellV2<T> {
    value: T,
}

impl<T: Copy> CellV2<T> {
    fn new(v: T) -> Self {
        Self { value: v }
    }
    fn set(&self, v: T) {
        unsafe {
            let p = &(self.value) as *const T as *mut T;
            *p = v;
        }
    }
    fn get(&self) -> T {
        self.value
    }
}

struct Table<'arg> {
    cell: CellV2<&'arg isize>,
}
fn evil<'long: 'short, 'short>(t: &Table<'long>, s: &'short isize) {
    // The following assignment is not legal, but it escapes from lifetime checking
    let u: &Table<'short> = t;
    u.cell.set(s);
}
fn innocent<'long>(t: &Table<'long>) {
    let foo: isize = 1;
    evil(t, &foo);
}
fn main() {
    let local = 100;
    let table = Table { cell: CellV2::new(&local) };
    innocent(&table);
    // reads `foo`, which has been destroyed
    let p = table.cell.get();
    println!("{}", p);
}
